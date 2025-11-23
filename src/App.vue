<template>
  <div class="app-wrapper">
    <!-- 背景装饰元素 -->
    <div class="bg-decoration decoration-1"></div>
    <div class="bg-decoration decoration-2"></div>
    <div class="bg-decoration decoration-3"></div>

    <div class="container">
      <!-- 头部 -->
      <header class="header">
        <div class="header-content">
          <div class="logo-section">
            <div class="logo-icon">✓</div>
            <h1>待办清单</h1>
          </div>
          <button class="settings-btn" title="设置" @click="settings.openSettings()">
            <svg viewBox="0 0 24 24" width="20" height="20" stroke="currentColor" fill="none">
              <path d="M12 15a3 3 0 100-6 3 3 0 000 6z" stroke-width="2" stroke-linecap="round"/>
              <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>
      </header>

      <!-- 加载状态 -->
      <div v-if="todos.isLoading" class="status-message loading">
        <div class="loading-spinner">⏳</div>
        <p>正在加载待办事项...</p>
      </div>

      <!-- 主要内容区域 -->
      <main v-else>
        <!-- 输入区域 -->
        <TodoInput @add="todos.addTodo" />

        <!-- 过滤器 -->
        <FilterBar
          :current-filter="todos.currentFilter"
          :stats="todos.stats"
          @filter="todos.setFilter"
        />

        <!-- 待办事项列表 -->
        <TodoList
          :todos="todos.filteredTodos"
          @toggle="todos.toggleTodo"
          @delete="todos.deleteTodo"
        />

        <!-- 底部统计和操作 -->
        <footer class="footer">
          <div class="stats-info">
            <span class="stats-icon">📊</span>
            <span>{{ todos.stats.active }} 个待办事项</span>
          </div>
          <button class="clear-btn" @click="todos.clearCompleted">
            <svg viewBox="0 0 24 24" width="16" height="16" stroke="currentColor" fill="none">
              <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <span>清除已完成</span>
          </button>
        </footer>
      </main>
    </div>

    <!-- 设置模态框 -->
    <SettingsModal
      :is-open="settings.isOpen"
      :config="settings.config"
      @close="settings.closeSettings"
      @save="settings.saveSettings"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import TodoInput from '@/components/TodoInput.vue'
import TodoList from '@/components/TodoList.vue'
import FilterBar from '@/components/FilterBar.vue'
import SettingsModal from '@/components/SettingsModal.vue'
import { useTodos } from '@/composables/useTodos'
import { useSettings } from '@/composables/useSettings'

const todos = useTodos()
const settings = useSettings()

onMounted(async () => {
  // Wait for Tauri API to be available
  if (typeof window !== 'undefined' && window.__TAURI__) {
    // Give Tauri some time to initialize
    await new Promise(resolve => setTimeout(resolve, 100))
  }
  await todos.loadTodos()
})
</script>
