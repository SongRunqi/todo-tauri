import type { ToastType } from '@/types'

export function useToast() {
  const showToast = (message: string, type: ToastType = 'info') => {
    // 创建toast元素
    const toast = document.createElement('div')
    toast.className = `toast toast-${type}`
    toast.textContent = message

    const bgColors = {
      success: '#10b981',
      error: '#ef4444',
      info: '#6b7280',
    }

    toast.style.cssText = `
      position: fixed;
      bottom: 24px;
      right: 24px;
      padding: 12px 20px;
      background: ${bgColors[type]};
      color: white;
      border-radius: 12px;
      font-size: 14px;
      font-weight: 600;
      box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
      z-index: 9999;
      animation: slideInUp 0.3s ease-out;
    `

    document.body.appendChild(toast)

    // 3秒后移除
    setTimeout(() => {
      toast.style.animation = 'slideOutDown 0.3s ease-out'
      setTimeout(() => {
        if (toast.parentNode) {
          document.body.removeChild(toast)
        }
      }, 300)
    }, 3000)
  }

  return {
    showToast,
  }
}
