import type { Todo, AppConfig } from '@/types'

const { invoke } = window.__TAURI__.tauri

export function useTauri() {
  const loadTodos = async (): Promise<Todo[]> => {
    return await invoke<Todo[]>('load_todos')
  }

  const addTodo = async (text: string): Promise<Todo> => {
    return await invoke<Todo>('add_todo', { text })
  }

  const toggleTodo = async (id: number): Promise<void> => {
    await invoke('toggle_todo', { id })
  }

  const deleteTodo = async (id: number): Promise<void> => {
    await invoke('delete_todo', { id })
  }

  const clearCompleted = async (): Promise<void> => {
    await invoke('clear_completed')
  }

  const getConfig = async (): Promise<AppConfig> => {
    return await invoke<AppConfig>('get_config')
  }

  const saveAppConfig = async (config: AppConfig): Promise<void> => {
    await invoke('save_app_config', { config })
  }

  return {
    loadTodos,
    addTodo,
    toggleTodo,
    deleteTodo,
    clearCompleted,
    getConfig,
    saveAppConfig,
  }
}
