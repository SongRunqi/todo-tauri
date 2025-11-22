// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
    next_id: Mutex<u32>,
}

fn get_data_path() -> PathBuf {
    let mut path = PathBuf::from(".");
    path.push("todos.json");
    path
}

#[tauri::command]
fn load_todos(state: State<AppState>) -> Result<Vec<Todo>, String> {
    let data_path = get_data_path();

    if data_path.exists() {
        let content = fs::read_to_string(&data_path)
            .map_err(|e| format!("无法读取文件: {}", e))?;

        let loaded_todos: Vec<Todo> = serde_json::from_str(&content)
            .map_err(|e| format!("无法解析 JSON: {}", e))?;

        if let Some(max_id) = loaded_todos.iter().map(|t| t.id).max() {
            *state.next_id.lock().unwrap() = max_id + 1;
        }

        *state.todos.lock().unwrap() = loaded_todos.clone();
        Ok(loaded_todos)
    } else {
        Ok(Vec::new())
    }
}

#[tauri::command]
fn add_todo(text: String, state: State<AppState>) -> Result<Todo, String> {
    let mut todos = state.todos.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let new_todo = Todo {
        id: *next_id,
        text,
        completed: false,
    };

    *next_id += 1;
    todos.push(new_todo.clone());

    save_todos(&todos)?;
    Ok(new_todo)
}

#[tauri::command]
fn toggle_todo(id: u32, state: State<AppState>) -> Result<(), String> {
    let mut todos = state.todos.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        save_todos(&todos)?;
        Ok(())
    } else {
        Err("未找到待办事项".to_string())
    }
}

#[tauri::command]
fn delete_todo(id: u32, state: State<AppState>) -> Result<(), String> {
    let mut todos = state.todos.lock().unwrap();
    todos.retain(|t| t.id != id);
    save_todos(&todos)?;
    Ok(())
}

#[tauri::command]
fn clear_completed(state: State<AppState>) -> Result<(), String> {
    let mut todos = state.todos.lock().unwrap();
    todos.retain(|t| !t.completed);
    save_todos(&todos)?;
    Ok(())
}

fn save_todos(todos: &Vec<Todo>) -> Result<(), String> {
    let data_path = get_data_path();
    let json = serde_json::to_string_pretty(todos)
        .map_err(|e| format!("无法序列化数据: {}", e))?;

    fs::write(&data_path, json)
        .map_err(|e| format!("无法保存文件: {}", e))?;

    Ok(())
}

fn main() {
    let app_state = AppState {
        todos: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            load_todos,
            add_todo,
            toggle_todo,
            delete_todo,
            clear_completed
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}
