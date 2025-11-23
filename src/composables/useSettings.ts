import { ref } from 'vue'
import type { AppConfig } from '@/types'
import { useTauri } from './useTauri'
import { useToast } from './useToast'

export function useSettings() {
  const tauri = useTauri()
  const { showToast } = useToast()

  const isOpen = ref(false)
  const config = ref<AppConfig>({
    apiKey: null,
    language: 'zh',
    llmBaseUrl: null,
    llmModel: null,
  })

  const openSettings = async () => {
    try {
      config.value = await tauri.getConfig()
      isOpen.value = true
    } catch (error) {
      console.error('加载配置失败:', error)
      showToast('加载配置失败', 'error')
    }
  }

  const closeSettings = () => {
    isOpen.value = false
  }

  const saveSettings = async () => {
    try {
      await tauri.saveAppConfig(config.value)
      showToast('配置已保存', 'success')
      setTimeout(() => {
        closeSettings()
      }, 1500)
    } catch (error) {
      console.error('保存配置失败:', error)
      showToast(`保存失败: ${error}`, 'error')
    }
  }

  return {
    isOpen,
    config,
    openSettings,
    closeSettings,
    saveSettings,
  }
}
