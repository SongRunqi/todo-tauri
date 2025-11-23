// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
struct AlfredItem {
    title: String,
    subtitle: Option<String>,
    arg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AlfredResponse {
    items: Vec<AlfredItem>,
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

fn get_todo_binary_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    // Determine the correct binary name based on the platform
    let binary_name = if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "binaries/todo-aarch64-apple-darwin"
        } else {
            "binaries/todo-x86_64-apple-darwin"
        }
    } else if cfg!(target_os = "linux") {
        "binaries/todo-x86_64-unknown-linux-gnu"
    } else {
        "binaries/todo"
    };

    // Use Tauri's resource API to resolve the binary path
    let resource_path = app
        .path_resolver()
        .resolve_resource(binary_name)
        .ok_or(format!("Failed to resolve {} binary path", binary_name))?;
    Ok(resource_path)
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
        let default_todos = r#"{"items":[]}"#;
        fs::write(&todos_file, default_todos).map_err(|e| format!("无法创建 todos.json: {}", e))?;
    }

    Ok(())
}

fn execute_todo_command(app: &tauri::AppHandle, args: &[&str]) -> Result<String, String> {
    let binary_path = get_todo_binary_path(app)?;
    let config = load_config(app);

    let mut cmd = Command::new(binary_path);
    cmd.args(args);

    // 设置语言
    if let Some(lang) = &config.language {
        cmd.env("TODO_LANG", lang);
    }

    // 设置 API Key
    if let Some(api_key) = &config.api_key {
        cmd.env("API_KEY", api_key);
    }

    // 设置 LLM Base URL
    if let Some(base_url) = &config.llm_base_url {
        if !base_url.is_empty() {
            cmd.env("LLM_BASE_URL", base_url);
        }
    }

    // 设置 LLM Model
    if let Some(model) = &config.llm_model {
        if !model.is_empty() {
            cmd.env("LLM_MODEL", model);
        }
    }

    let output = cmd.output().map_err(|e| format!("执行命令失败: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("命令执行失败: {}", error));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("解析输出失败: {}", e))
}

fn parse_alfred_output(output: &str) -> Result<Vec<Todo>, String> {
    // 处理空输出或无效JSON
    let output = output.trim();
    if output.is_empty() || output == "{\"items\":[]}" {
        return Ok(Vec::new());
    }

    let alfred_response: AlfredResponse = serde_json::from_str(output)
        .map_err(|e| format!("解析 JSON 失败: {} (输出: {})", e, output))?;

    let mut todos = Vec::new();
    for item in alfred_response.items {
        // 从 title 中提取 ID 和 text
        // 格式可能是: "[1] 🎯 Task Name [剩余时间]" 或 "[1] Task Name" 等多种格式
        let title_parts: Vec<&str> = item.title.splitn(2, ']').collect();
        if title_parts.len() < 2 {
            continue;
        }

        let id_str = title_parts[0].trim_start_matches('[').trim();
        let id = id_str.parse::<u32>().unwrap_or(0);
        if id == 0 {
            continue;
        }

        let rest = title_parts[1].trim();

        // 尝试提取任务名称，支持多种格式
        let text = if let Some(pos) = rest.find("🎯") {
            rest[pos + 3..]
                .trim()
                .split('[')
                .next()
                .unwrap_or("")
                .trim()
        } else if let Some(pos) = rest.find("✅") {
            rest[pos + 3..]
                .trim()
                .split('[')
                .next()
                .unwrap_or("")
                .trim()
        } else {
            rest.split('[').next().unwrap_or("").trim()
        };

        if text.is_empty() {
            continue;
        }

        let completed = item.subtitle.as_ref().map_or(false, |s| s.contains("✅"));

        todos.push(Todo {
            id,
            text: text.to_string(),
            status: if completed {
                "completed".to_string()
            } else {
                "pending".to_string()
            },
            description: item.subtitle.clone(),
            due_date: None,
            urgent: None,
            completed,
        });
    }

    Ok(todos)
}

#[tauri::command]
fn load_todos(app: tauri::AppHandle) -> Result<Vec<Todo>, String> {
    let output = execute_todo_command(&app, &["list"])?;
    parse_alfred_output(&output)
}

#[tauri::command]
fn add_todo(text: String, app: tauri::AppHandle) -> Result<Todo, String> {
    // 使用自然语言创建任务
    execute_todo_command(&app, &[&text])?;

    // 重新加载列表以获取新创建的任务
    let todos = load_todos(app)?;
    todos
        .into_iter()
        .max_by_key(|t| t.id)
        .ok_or_else(|| "未能创建任务".to_string())
}

#[tauri::command]
fn toggle_todo(id: u32, app: tauri::AppHandle) -> Result<(), String> {
    let id_str = id.to_string();
    execute_todo_command(&app, &["complete", &id_str])?;
    Ok(())
}

#[tauri::command]
fn delete_todo(id: u32, app: tauri::AppHandle) -> Result<(), String> {
    let id_str = id.to_string();
    execute_todo_command(&app, &["delete", &id_str])?;
    Ok(())
}

#[tauri::command]
fn clear_completed(app: tauri::AppHandle) -> Result<(), String> {
    // 获取所有已完成的任务并删除
    let output = execute_todo_command(&app, &["back"])?;
    let alfred_response: AlfredResponse =
        serde_json::from_str(&output).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    for item in alfred_response.items {
        if let Some(arg) = item.arg {
            let _ = execute_todo_command(&app, &["delete", &arg]);
        }
    }

    Ok(())
}

#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<AppConfig, String> {
    Ok(load_config(&app))
}

#[tauri::command]
fn save_app_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    save_config(&app, &config)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化 todo 文件目录
            initialize_todo_files()
                .map_err(|e| tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

            // 获取 todo 二进制文件路径
            let binary_path = get_todo_binary_path(app)
                .map_err(|e| tauri::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    e,
                )))?;

            // 初始化 go-todo
            let output = Command::new(&binary_path)
                .arg("init")
                .env("TODO_LANG", "zh")
                .output();

            if let Err(e) = output {
                eprintln!("初始化 go-todo 失败: {}", e);
            }

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
