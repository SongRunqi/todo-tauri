import type { Todo, AppConfig } from '@/types'

// Helper function to wait for Tauri API to be available
async function waitForTauriAPI(maxWaitMs = 5000): Promise<any> {
  const startTime = Date.now()

  while (Date.now() - startTime < maxWaitMs) {
    if (typeof window !== 'undefined' && window.__TAURI__?.tauri?.invoke) {
      return window.__TAURI__.tauri.invoke
    }
    // Wait 50ms before checking again
    await new Promise(resolve => setTimeout(resolve, 50))
  }

  throw new Error('Tauri API is not available after waiting')
}

export function useTauri() {
  const loadTodos = async (): Promise<Todo[]> => {
    const invoke = await waitForTauriAPI()
    return await invoke<Todo[]>('load_todos')
  }

  const addTodo = async (text: string): Promise<Todo> => {
    const invoke = await waitForTauriAPI()
    return await invoke<Todo>('add_todo', { text })
  }

  const toggleTodo = async (id: number): Promise<void> => {
    const invoke = await waitForTauriAPI()
    await invoke('toggle_todo', { id })
  }

  const deleteTodo = async (id: number): Promise<void> => {
    const invoke = await waitForTauriAPI()
    await invoke('delete_todo', { id })
  }

  const clearCompleted = async (): Promise<void> => {
    const invoke = await waitForTauriAPI()
    await invoke('clear_completed')
  }

  const getConfig = async (): Promise<AppConfig> => {
    const invoke = await waitForTauriAPI()
    return await invoke<AppConfig>('get_config')
  }

  const saveAppConfig = async (config: AppConfig): Promise<void> => {
    const invoke = await waitForTauriAPI()
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
