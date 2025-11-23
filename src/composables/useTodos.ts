import { ref, computed } from 'vue'
import type { Todo, FilterType } from '@/types'
import { useTauri } from './useTauri'
import { useToast } from './useToast'

const todos = ref<Todo[]>([])
const currentFilter = ref<FilterType>('all')
const isLoading = ref(false)

export function useTodos() {
  const tauri = useTauri()
  const { showToast } = useToast()

  const filteredTodos = computed(() => {
    switch (currentFilter.value) {
      case 'active':
        return todos.value.filter(t => !t.completed)
      case 'completed':
        return todos.value.filter(t => t.completed)
      default:
        return todos.value
    }
  })

  const stats = computed(() => ({
    total: todos.value.length,
    active: todos.value.filter(t => !t.completed).length,
    completed: todos.value.filter(t => t.completed).length,
  }))

  const loadTodos = async () => {
    isLoading.value = true
    try {
      todos.value = await tauri.loadTodos()
      showToast('加载成功', 'success')
    } catch (error) {
      console.error('加载待办事项失败:', error)
      todos.value = []
      showToast(`加载失败: ${error}`, 'error')
    } finally {
      isLoading.value = false
    }
  }

  const addTodo = async (text: string) => {
    if (!text.trim()) return

    try {
      const newTodo = await tauri.addTodo(text)
      todos.value.push(newTodo)
      showToast('添加成功', 'success')
    } catch (error) {
      console.error('添加待办事项失败:', error)
      showToast(`添加失败: ${error}`, 'error')
    }
  }

  const toggleTodo = async (id: number) => {
    try {
      await tauri.toggleTodo(id)
      const todo = todos.value.find(t => t.id === id)
      if (todo) {
        todo.completed = !todo.completed
      }
    } catch (error) {
      console.error('切换状态失败:', error)
      showToast('操作失败', 'error')
    }
  }

  const deleteTodo = async (id: number) => {
    try {
      await tauri.deleteTodo(id)
      todos.value = todos.value.filter(t => t.id !== id)
      showToast('删除成功', 'success')
    } catch (error) {
      console.error('删除待办事项失败:', error)
      showToast('删除失败', 'error')
    }
  }

  const clearCompleted = async () => {
    const completedCount = stats.value.completed
    if (completedCount === 0) {
      showToast('没有已完成的待办事项', 'info')
      return
    }

    try {
      await tauri.clearCompleted()
      todos.value = todos.value.filter(t => !t.completed)
      showToast(`已清除 ${completedCount} 个已完成事项`, 'success')
    } catch (error) {
      console.error('清除已完成项失败:', error)
      showToast('清除失败', 'error')
    }
  }

  const setFilter = (filter: FilterType) => {
    currentFilter.value = filter
  }

  return {
    todos,
    filteredTodos,
    currentFilter,
    stats,
    isLoading,
    loadTodos,
    addTodo,
    toggleTodo,
    deleteTodo,
    clearCompleted,
    setFilter,
  }
}
