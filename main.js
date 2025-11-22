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

// 初始化
async function init() {
    try {
        todos = await invoke('load_todos');
        renderTodos();
    } catch (error) {
        console.error('加载待办事项失败:', error);
        todos = [];
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
