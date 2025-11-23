<template>
  <div class="input-section">
    <div class="input-wrapper">
      <svg class="input-icon" viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" fill="none">
        <circle cx="12" cy="12" r="10" stroke-width="2"/>
        <path d="M12 8v8M8 12h8" stroke-width="2" stroke-linecap="round"/>
      </svg>
      <input
        v-model="inputText"
        type="text"
        placeholder="添加新的待办事项..."
        autocomplete="off"
        @keypress.enter="handleAdd"
      >
      <button class="add-btn" @click="handleAdd">
        <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" fill="none">
          <path d="M12 5v14M5 12h14" stroke-width="2.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const inputText = ref('')

const emit = defineEmits<{
  add: [text: string]
}>()

const handleAdd = () => {
  const text = inputText.value.trim()
  if (!text) return

  emit('add', text)
  inputText.value = ''
}
</script>
