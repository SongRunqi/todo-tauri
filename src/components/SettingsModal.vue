<template>
  <div v-if="isOpen" class="modal show" @click.self="$emit('close')">
    <div class="modal-content">
      <div class="modal-header">
        <h2>⚙️ 设置</h2>
        <button class="close-btn" @click="$emit('close')">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label for="apiKey">API 密钥 (API_KEY)</label>
          <input
            id="apiKey"
            v-model="localConfig.apiKey"
            type="password"
            placeholder="输入你的 DeepSeek API 密钥"
            autocomplete="off"
          >
          <small>获取地址: <a href="https://platform.deepseek.com/" target="_blank">platform.deepseek.com</a></small>
        </div>

        <div class="form-group">
          <label for="language">语言 (TODO_LANG)</label>
          <select id="language" v-model="localConfig.language">
            <option value="zh">中文</option>
            <option value="en">English</option>
          </select>
        </div>

        <div class="form-group">
          <label for="llmBaseUrl">LLM API 端点 (可选)</label>
          <input
            id="llmBaseUrl"
            v-model="localConfig.llmBaseUrl"
            type="text"
            placeholder="https://api.deepseek.com/chat/completions"
            autocomplete="off"
          >
          <small>默认使用 DeepSeek,可切换到其他 LLM 提供商</small>
        </div>

        <div class="form-group">
          <label for="llmModel">模型 (可选)</label>
          <input
            id="llmModel"
            v-model="localConfig.llmModel"
            type="text"
            placeholder="deepseek-chat"
            autocomplete="off"
          >
        </div>
      </div>
      <div class="modal-footer">
        <button class="save-btn" @click="$emit('save', localConfig)">保存设置</button>
        <button class="cancel-btn" @click="$emit('close')">取消</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { watch, ref } from 'vue'
import type { AppConfig } from '@/types'

const props = defineProps<{
  isOpen: boolean
  config: AppConfig
}>()

const emit = defineEmits<{
  close: []
  save: [config: AppConfig]
}>()

const localConfig = ref<AppConfig>({ ...props.config })

// 当 props.config 改变时更新 localConfig
watch(() => props.config, (newConfig) => {
  localConfig.value = { ...newConfig }
}, { deep: true })

// ESC 键关闭
const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen) {
    emit('close')
  }
}

// 添加和移除键盘监听
watch(() => props.isOpen, (isOpen) => {
  if (isOpen) {
    document.addEventListener('keydown', handleKeyDown)
  } else {
    document.removeEventListener('keydown', handleKeyDown)
  }
})
</script>
