export interface Todo {
  id: number
  text: string
  completed: boolean
}

export type FilterType = 'all' | 'active' | 'completed'

export interface AppConfig {
  apiKey: string | null
  language: string
  llmBaseUrl: string | null
  llmModel: string | null
}

export type ToastType = 'success' | 'error' | 'info'
