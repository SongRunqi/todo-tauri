import type { Todo, AppConfig } from '@/types'

// Helper function to safely get the invoke function
function getInvoke() {
  if (typeof window !== 'undefined' && window.__TAURI__?.tauri?.invoke) {
    return window.__TAURI__.tauri.invoke
  }
  throw new Error('Tauri API is not available')
}

export function useTauri() {
  const loadTodos = async (): Promise<Todo[]> => {
    const invoke = getInvoke()
    return await invoke<Todo[]>('load_todos')
  }

  const addTodo = async (text: string): Promise<Todo> => {
    const invoke = getInvoke()
    return await invoke<Todo>('add_todo', { text })
  }

  const toggleTodo = async (id: number): Promise<void> => {
    const invoke = getInvoke()
    await invoke('toggle_todo', { id })
  }

  const deleteTodo = async (id: number): Promise<void> => {
    const invoke = getInvoke()
    await invoke('delete_todo', { id })
  }

  const clearCompleted = async (): Promise<void> => {
    const invoke = getInvoke()
    await invoke('clear_completed')
  }

  const getConfig = async (): Promise<AppConfig> => {
    const invoke = getInvoke()
    return await invoke<AppConfig>('get_config')
  }

  const saveAppConfig = async (config: AppConfig): Promise<void> => {
    const invoke = getInvoke()
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
