<template>
  <div class="todo-list-container">
    <ul v-if="todos.length > 0" class="todo-list">
      <TodoItem
        v-for="todo in todos"
        :key="todo.id"
        :todo="todo"
        @toggle="$emit('toggle', $event)"
        @delete="$emit('delete', $event)"
      />
    </ul>

    <!-- 空状态 -->
    <div v-else class="empty-state show">
      <div class="empty-icon">
        <svg viewBox="0 0 24 24" width="64" height="64" stroke="currentColor" fill="none">
          <path d="M9 11l3 3L22 4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M21 12v7a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2h11" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </div>
      <h3 class="empty-title">暂无待办事项</h3>
      <p class="empty-description">在上方输入框中添加你的第一个待办事项</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Todo } from '@/types'
import TodoItem from './TodoItem.vue'

defineProps<{
  todos: Todo[]
}>()

defineEmits<{
  toggle: [id: number]
  delete: [id: number]
}>()
</script>
