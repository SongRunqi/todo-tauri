<template>
  <li :class="{ 'todo-item': true, 'completed': todo.completed }">
    <input
      type="checkbox"
      class="todo-checkbox"
      :checked="todo.completed"
      @change="$emit('toggle', todo.id)"
    >
    <span class="todo-text">{{ todo.text }}</span>
    <button class="delete-btn" @click="$emit('delete', todo.id)">删除</button>
  </li>
</template>

<script setup lang="ts">
import type { Todo } from '@/types'

defineProps<{
  todo: Todo
}>()

defineEmits<{
  toggle: [id: number]
  delete: [id: number]
}>()
</script>
