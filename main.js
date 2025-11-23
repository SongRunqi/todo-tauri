const { invoke } = window.__TAURI__.tauri;

let todos = [];
let currentFilter = 'all';

// DOM 元素
const todoInput = document.getElementById('todoInput');
const addBtn = document.getElementById('addBtn');
const todoList = document.getElementById('todoList');
const todoCount = document.getElementById('todoCount');
const clearCompletedBtn = document.getElementById('clearCompleted');
const filterBtns = document.querySelectorAll('.filter-btn');
const statusMessage = document.getElementById('statusMessage');
const appContent = document.getElementById('appContent');
const emptyState = document.getElementById('emptyState');

// 过滤器计数元素
const countAll = document.getElementById('countAll');
const countActive = document.getElementById('countActive');
const countCompleted = document.getElementById('countCompleted');

// 显示状态消息
function showStatus(message, type = 'loading') {
    statusMessage.className = `status-message ${type}`;
    if (type === 'loading') {
        statusMessage.innerHTML = `<div class="loading-spinner">⏳</div><p>${message}</p>`;
    } else if (type === 'error') {
        statusMessage.innerHTML = `<p>❌ ${message}</p><button onclick="retryInit()">重试</button>`;
    } else if (type === 'success') {
        statusMessage.innerHTML = `<p>✅ ${message}</p>`;
    }
    statusMessage.style.display = 'flex';
}

// 隐藏状态消息
function hideStatus() {
    statusMessage.className = 'status-message';
    statusMessage.innerHTML = '';
    statusMessage.style.display = 'none';
}

// 重试初始化
async function retryInit() {
    init();
}

// 初始化
async function init() {
    try {
        showStatus('正在加载待办事项...');
        todos = await invoke('load_todos');
        hideStatus();
        renderTodos();
        showToast('加载成功', 'success');
    } catch (error) {
        console.error('加载待办事项失败:', error);
        // 即使加载失败，也显示UI，只是todos数组为空
        todos = [];
        hideStatus();
        renderTodos();
        showToast(`加载失败: ${error}`, 'error');
    }
}

// Toast 通知
function showToast(message, type = 'info') {
    // 创建toast元素
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    toast.textContent = message;
    toast.style.cssText = `
        position: fixed;
        bottom: 24px;
        right: 24px;
        padding: 12px 20px;
        background: ${type === 'success' ? '#10b981' : type === 'error' ? '#ef4444' : '#6b7280'};
        color: white;
        border-radius: 12px;
        font-size: 14px;
        font-weight: 600;
        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
        z-index: 9999;
        animation: slideInUp 0.3s ease-out;
    `;

    document.body.appendChild(toast);

    // 3秒后移除
    setTimeout(() => {
        toast.style.animation = 'slideOutDown 0.3s ease-out';
        setTimeout(() => {
            document.body.removeChild(toast);
        }, 300);
    }, 3000);
}

// 添加待办事项
async function addTodo() {
    const text = todoInput.value.trim();
    if (!text) return;

    try {
        const newTodo = await invoke('add_todo', { text });
        todos.push(newTodo);
        todoInput.value = '';
        renderTodos();
        showToast('添加成功', 'success');
    } catch (error) {
        console.error('添加待办事项失败:', error);
        showToast(`添加失败: ${error}`, 'error');
    }
}

// 切换完成状态
async function toggleTodo(id) {
    try {
        await invoke('toggle_todo', { id });
        const todo = todos.find(t => t.id === id);
        if (todo) {
            todo.completed = !todo.completed;
            renderTodos();
        }
    } catch (error) {
        console.error('切换状态失败:', error);
        showToast('操作失败', 'error');
    }
}

// 删除待办事项
async function deleteTodo(id) {
    try {
        await invoke('delete_todo', { id });
        todos = todos.filter(t => t.id !== id);
        renderTodos();
        showToast('删除成功', 'success');
    } catch (error) {
        console.error('删除待办事项失败:', error);
        showToast('删除失败', 'error');
    }
}

// 清除已完成
async function clearCompleted() {
    const completedCount = todos.filter(t => t.completed).length;
    if (completedCount === 0) {
        showToast('没有已完成的待办事项', 'info');
        return;
    }

    try {
        await invoke('clear_completed');
        todos = todos.filter(t => !t.completed);
        renderTodos();
        showToast(`已清除 ${completedCount} 个已完成事项`, 'success');
    } catch (error) {
        console.error('清除已完成项失败:', error);
        showToast('清除失败', 'error');
    }
}

// 渲染待办事项列表
function renderTodos() {
    const filteredTodos = getFilteredTodos();

    // 清空列表
    todoList.innerHTML = '';

    // 如果没有待办事项，显示空状态
    if (todos.length === 0) {
        emptyState.classList.add('show');
        todoList.style.display = 'none';
    } else {
        emptyState.classList.remove('show');
        todoList.style.display = 'block';

        // 渲染每个待办事项
        filteredTodos.forEach(todo => {
            const li = document.createElement('li');
            li.className = `todo-item ${todo.completed ? 'completed' : ''}`;
            li.innerHTML = `
                <input
                    type="checkbox"
                    class="todo-checkbox"
                    ${todo.completed ? 'checked' : ''}
                    onchange="toggleTodo(${todo.id})"
                >
                <span class="todo-text">${escapeHtml(todo.text)}</span>
                <button class="delete-btn" onclick="deleteTodo(${todo.id})">删除</button>
            `;
            todoList.appendChild(li);
        });
    }

    updateStats();
    updateFilterCounts();
}

// 获取过滤后的待办事项
function getFilteredTodos() {
    switch (currentFilter) {
        case 'active':
            return todos.filter(t => !t.completed);
        case 'completed':
            return todos.filter(t => t.completed);
        default:
            return todos;
    }
}

// 更新统计信息
function updateStats() {
    const activeCount = todos.filter(t => !t.completed).length;
    todoCount.textContent = `${activeCount} 个待办事项`;
}

// 更新过滤器计数
function updateFilterCounts() {
    const totalCount = todos.length;
    const activeCount = todos.filter(t => !t.completed).length;
    const completedCount = todos.filter(t => t.completed).length;

    countAll.textContent = totalCount;
    countActive.textContent = activeCount;
    countCompleted.textContent = completedCount;
}

// HTML 转义
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// 设置相关 DOM 元素
const settingsBtn = document.getElementById('settingsBtn');
const settingsModal = document.getElementById('settingsModal');
const closeSettings = document.getElementById('closeSettings');
const saveSettings = document.getElementById('saveSettings');
const cancelSettings = document.getElementById('cancelSettings');
const configStatus = document.getElementById('configStatus');

// 设置表单元素
const apiKeyInput = document.getElementById('apiKey');
const languageSelect = document.getElementById('language');
const llmBaseUrlInput = document.getElementById('llmBaseUrl');
const llmModelInput = document.getElementById('llmModel');

// 打开设置模态框
async function openSettings() {
    try {
        const config = await invoke('get_config');

        // 填充表单
        apiKeyInput.value = config.apiKey || '';
        languageSelect.value = config.language || 'zh';
        llmBaseUrlInput.value = config.llmBaseUrl || '';
        llmModelInput.value = config.llmModel || '';

        // 显示模态框
        settingsModal.classList.add('show');
        configStatus.className = 'config-status';
    } catch (error) {
        console.error('加载配置失败:', error);
        showToast('加载配置失败', 'error');
    }
}

// 关闭设置模态框
function closeSettingsModal() {
    settingsModal.classList.remove('show');
    configStatus.className = 'config-status';
}

// 保存设置
async function saveSettingsConfig() {
    try {
        const config = {
            apiKey: apiKeyInput.value.trim() || null,
            language: languageSelect.value,
            llmBaseUrl: llmBaseUrlInput.value.trim() || null,
            llmModel: llmModelInput.value.trim() || null,
        };

        await invoke('save_app_config', { config });

        // 显示成功消息
        configStatus.className = 'config-status success';
        configStatus.textContent = '✓ 配置保存成功！';

        // 1.5秒后关闭模态框
        setTimeout(() => {
            closeSettingsModal();
            showToast('配置已保存', 'success');
        }, 1500);
    } catch (error) {
        console.error('保存配置失败:', error);
        configStatus.className = 'config-status error';
        configStatus.textContent = '✗ 保存失败: ' + error;
    }
}

// 设置相关事件监听
settingsBtn.addEventListener('click', openSettings);
closeSettings.addEventListener('click', closeSettingsModal);
cancelSettings.addEventListener('click', closeSettingsModal);
saveSettings.addEventListener('click', saveSettingsConfig);

// 点击模态框背景关闭
settingsModal.addEventListener('click', (e) => {
    if (e.target === settingsModal) {
        closeSettingsModal();
    }
});

// ESC 键关闭模态框
document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && settingsModal.classList.contains('show')) {
        closeSettingsModal();
    }
});

// 事件监听
addBtn.addEventListener('click', addTodo);
todoInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') addTodo();
});

clearCompletedBtn.addEventListener('click', clearCompleted);

filterBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        filterBtns.forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        currentFilter = btn.dataset.filter;
        renderTodos();
    });
});

// 添加toast动画样式
const style = document.createElement('style');
style.textContent = `
@keyframes slideInUp {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

@keyframes slideOutDown {
    from {
        opacity: 1;
        transform: translateY(0);
    }
    to {
        opacity: 0;
        transform: translateY(20px);
    }
}
`;
document.head.appendChild(style);

// 启动应用
init();
