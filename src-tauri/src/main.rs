// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    #[serde(rename = "taskId")]
    id: u32,
    #[serde(rename = "taskName")]
    text: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "taskDesc")]
    description: Option<String>,
    #[serde(rename = "dueDate")]
    due_date: Option<String>,
    #[serde(rename = "urgent")]
    urgent: Option<String>,
    #[serde(default)]
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoFile {
    items: Vec<Todo>,
}

impl Default for TodoFile {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    #[serde(rename = "apiKey")]
    api_key: Option<String>,
    #[serde(rename = "language")]
    language: Option<String>,
    #[serde(rename = "llmBaseUrl")]
    llm_base_url: Option<String>,
    #[serde(rename = "llmModel")]
    llm_model: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            language: Some("zh".to_string()),
            llm_base_url: None,
            llm_model: None,
        }
    }
}

fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to get app data directory")?;

    // 确保目录存在
    fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;

    Ok(app_dir.join("config.json"))
}

fn load_config(app: &tauri::AppHandle) -> AppConfig {
    match get_config_path(app) {
        Ok(config_path) => {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                        return config;
                    }
                }
            }
        }
        Err(_) => {}
    }
    AppConfig::default()
}

fn save_config(app: &tauri::AppHandle, config: &AppConfig) -> Result<(), String> {
    let config_path = get_config_path(app)?;
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&config_path, json).map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}

fn read_todos() -> Result<TodoFile, String> {
    let todos_file = get_todos_file_path()?;

    if !todos_file.exists() {
        return Ok(TodoFile::default());
    }

    let content = fs::read_to_string(&todos_file)
        .map_err(|e| format!("无法读取 todos.json: {}", e))?;

    let todo_file: TodoFile = serde_json::from_str(&content)
        .map_err(|e| format!("解析 todos.json 失败: {}", e))?;

    Ok(todo_file)
}

fn write_todos(todo_file: &TodoFile) -> Result<(), String> {
    let todos_file = get_todos_file_path()?;
    let json = serde_json::to_string_pretty(todo_file)
        .map_err(|e| format!("序列化 todos 失败: {}", e))?;

    fs::write(&todos_file, json)
        .map_err(|e| format!("写入 todos.json 失败: {}", e))?;

    Ok(())
}

fn get_next_id(todo_file: &TodoFile) -> u32 {
    todo_file.items.iter()
        .map(|t| t.id)
        .max()
        .unwrap_or(0) + 1
}

fn get_todo_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let todo_dir = home.join(".todo");

    // 确保 .todo 目录存在
    fs::create_dir_all(&todo_dir).map_err(|e| format!("无法创建 .todo 目录: {}", e))?;

    Ok(todo_dir)
}

fn get_todos_file_path() -> Result<PathBuf, String> {
    let todo_dir = get_todo_dir()?;
    Ok(todo_dir.join("todos.json"))
}

fn initialize_todo_files() -> Result<(), String> {
    let todos_file = get_todos_file_path()?;

    // 如果 todos.json 不存在，创建一个空的
    if !todos_file.exists() {
        let default_todos = TodoFile::default();
        write_todos(&default_todos)?;
    }

    Ok(())
}

#[tauri::command]
fn load_todos(_app: tauri::AppHandle) -> Result<Vec<Todo>, String> {
    println!("\n📋 [load_todos] 加载待办事项列表");
    let todo_file = read_todos()?;
    println!("   ✓ 成功加载 {} 个待办事项\n", todo_file.items.len());
    Ok(todo_file.items)
}

#[tauri::command]
fn add_todo(text: String, _app: tauri::AppHandle) -> Result<Todo, String> {
    println!("\n➕ [add_todo] 添加新待办事项: \"{}\"", text);

    let mut todo_file = read_todos()?;
    let new_id = get_next_id(&todo_file);

    let new_todo = Todo {
        id: new_id,
        text: text.trim().to_string(),
        status: "pending".to_string(),
        description: None,
        due_date: None,
        urgent: None,
        completed: false,
    };

    todo_file.items.push(new_todo.clone());
    write_todos(&todo_file)?;

    println!("   ✓ 任务创建成功 (ID: {})\n", new_todo.id);
    Ok(new_todo)
}

#[tauri::command]
fn toggle_todo(id: u32, _app: tauri::AppHandle) -> Result<(), String> {
    println!("\n✅ [toggle_todo] 切换待办事项状态 (ID: {})", id);

    let mut todo_file = read_todos()?;

    if let Some(todo) = todo_file.items.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        todo.status = if todo.completed {
            "completed".to_string()
        } else {
            "pending".to_string()
        };

        let status = todo.status.clone(); // Capture status before dropping mutable borrow

        write_todos(&todo_file)?;
        println!("   ✓ 状态切换成功 (新状态: {})\n", status);
        Ok(())
    } else {
        Err(format!("未找到 ID 为 {} 的待办事项", id))
    }
}

#[tauri::command]
fn delete_todo(id: u32, _app: tauri::AppHandle) -> Result<(), String> {
    println!("\n🗑️  [delete_todo] 删除待办事项 (ID: {})", id);

    let mut todo_file = read_todos()?;
    let original_len = todo_file.items.len();

    todo_file.items.retain(|t| t.id != id);

    if todo_file.items.len() == original_len {
        return Err(format!("未找到 ID 为 {} 的待办事项", id));
    }

    write_todos(&todo_file)?;
    println!("   ✓ 删除成功\n");
    Ok(())
}

#[tauri::command]
fn clear_completed(_app: tauri::AppHandle) -> Result<(), String> {
    println!("\n🧹 [clear_completed] 清除已完成的待办事项");

    let mut todo_file = read_todos()?;
    let original_len = todo_file.items.len();

    todo_file.items.retain(|t| !t.completed);

    let removed_count = original_len - todo_file.items.len();
    println!("   📊 清除了 {} 个已完成任务", removed_count);

    write_todos(&todo_file)?;
    println!("   ✓ 清除完成\n");
    Ok(())
}

#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<AppConfig, String> {
    println!("\n⚙️  [get_config] 获取应用配置");
    let config = load_config(&app);
    println!("   ✓ 配置加载成功\n");
    Ok(config)
}

#[tauri::command]
fn save_app_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    println!("\n💾 [save_app_config] 保存应用配置");
    save_config(&app, &config)?;
    println!("   ✓ 配置保存成功\n");
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            println!("====================================");
            println!("🚀 正在启动 Tauri Todo 应用...");
            println!("====================================");

            // 初始化 todo 文件目录
            println!("📁 初始化 todo 文件目录...");
            match initialize_todo_files() {
                Ok(_) => println!("   ✓ todo 文件目录初始化成功"),
                Err(e) => {
                    eprintln!("   ✗ 初始化失败: {}", e);
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
                }
            }

            println!("====================================");
            println!("✅ 应用启动成功！");
            println!("====================================\n");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_todos,
            add_todo,
            toggle_todo,
            delete_todo,
            clear_completed,
            get_config,
            save_app_config
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
