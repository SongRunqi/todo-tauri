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
    fs::write(&config_path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}

fn get_todo_binary_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    // Use Tauri's sidecar API to automatically select the correct binary for the platform
    let resource_path = app
        .path_resolver()
        .resolve_resource("binaries/todo")
        .ok_or("Failed to resolve todo binary path")?;
    Ok(resource_path)
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

    let output = cmd
        .output()
        .map_err(|e| format!("执行命令失败: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("命令执行失败: {}", error));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("解析输出失败: {}", e))
}

fn parse_alfred_output(output: &str) -> Result<Vec<Todo>, String> {
    let alfred_response: AlfredResponse =
        serde_json::from_str(output).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut todos = Vec::new();
    for item in alfred_response.items {
        // 从 title 中提取 ID 和 text
        // 格式: "[1] 🎯 Task Name [剩余时间]"
        let title_parts: Vec<&str> = item.title.splitn(2, ']').collect();
        if title_parts.len() < 2 {
            continue;
        }

        let id_str = title_parts[0].trim_start_matches('[');
        let id = id_str.parse::<u32>().unwrap_or(0);

        let rest = title_parts[1].trim();
        let text_parts: Vec<&str> = rest.splitn(2, "🎯").collect();
        let text = if text_parts.len() > 1 {
            text_parts[1].trim().split('[').next().unwrap_or("").trim()
        } else {
            rest.split('[').next().unwrap_or("").trim()
        };

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
            // 初始化 todo 环境
            let binary_path = app
                .path_resolver()
                .resolve_resource("binaries/todo")
                .ok_or("Failed to resolve todo binary")?;

            // 初始化 go-todo
            let _ = Command::new(binary_path)
                .arg("init")
                .env("TODO_LANG", "zh")
                .output();

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
