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
}

// 隐藏状态消息
function hideStatus() {
    statusMessage.className = 'status-message';
    statusMessage.innerHTML = '';
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
        appContent.style.display = '';
        renderTodos();
    } catch (error) {
        console.error('加载待办事项失败:', error);
        showStatus(`加载失败: ${error}`, 'error');
        todos = [];
        appContent.style.display = 'none';
    }
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
    } catch (error) {
        console.error('添加待办事项失败:', error);
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
    }
}

// 删除待办事项
async function deleteTodo(id) {
    try {
        await invoke('delete_todo', { id });
        todos = todos.filter(t => t.id !== id);
        renderTodos();
    } catch (error) {
        console.error('删除待办事项失败:', error);
    }
}

// 清除已完成
async function clearCompleted() {
    try {
        await invoke('clear_completed');
        todos = todos.filter(t => !t.completed);
        renderTodos();
    } catch (error) {
        console.error('清除已完成项失败:', error);
    }
}

// 渲染待办事项列表
function renderTodos() {
    const filteredTodos = getFilteredTodos();

    todoList.innerHTML = '';
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

    updateStats();
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

        // 2秒后关闭模态框
        setTimeout(() => {
            closeSettingsModal();
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

// 启动应用
init();
