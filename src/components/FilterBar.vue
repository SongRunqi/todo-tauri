<template>
  <div class="filter-section">
    <button
      :class="{ 'filter-btn': true, 'active': currentFilter === 'all' }"
      @click="$emit('filter', 'all')"
    >
      <span class="filter-count">{{ stats.total }}</span>
      <span class="filter-label">全部</span>
    </button>
    <button
      :class="{ 'filter-btn': true, 'active': currentFilter === 'active' }"
      @click="$emit('filter', 'active')"
    >
      <span class="filter-count">{{ stats.active }}</span>
      <span class="filter-label">进行中</span>
    </button>
    <button
      :class="{ 'filter-btn': true, 'active': currentFilter === 'completed' }"
      @click="$emit('filter', 'completed')"
    >
      <span class="filter-count">{{ stats.completed }}</span>
      <span class="filter-label">已完成</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { FilterType } from '@/types'

interface Stats {
  total: number
  active: number
  completed: number
}

defineProps<{
  currentFilter: FilterType
  stats: Stats
}>()

defineEmits<{
  filter: [filter: FilterType]
}>()
</script>
