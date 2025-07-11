import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/api'
import { message } from 'ant-design-vue'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref({
    system: {
      system_name: 'Rust Admin',
      system_description: '基于 Tauri + Vue 3 的后台管理系统',
      system_version: '1.0.0'
    },
    ui: {
      theme_color: '#1890ff',
      language: 'zh-CN',
      page_size: 10
    },
    security: {
      enable_registration: false,
      session_timeout: 3600,
      max_login_attempts: 5,
      maintenance_mode: false
    }
  })

  const loading = ref(false)

  // 加载所有设置
  const loadSettings = async () => {
    loading.value = true
    try {
      const response = await api.getAllSettings()
      if (response.success) {
        settings.value = response.data
        
        // 应用设置
        applyUISettings()
        
        return response.data
      }
    } catch (error) {
      console.error('加载设置失败:', error)
    } finally {
      loading.value = false
    }
  }

  // 应用界面设置
  const applyUISettings = () => {
    // 应用主题色
    applyThemeColor(settings.value.ui.theme_color)
    
    // 应用语言设置
    applyLanguage(settings.value.ui.language)
    
    // 更新页面标题
    document.title = settings.value.system.system_name
  }

  // 应用主题色
  const applyThemeColor = (color) => {
    // 更新 CSS 变量
    document.documentElement.style.setProperty('--primary-color', color)
    
    // 更新 Ant Design 主题色
    const styleElement = document.createElement('style')
    styleElement.innerHTML = `
      .ant-btn-primary {
        background-color: ${color} !important;
        border-color: ${color} !important;
      }
      .ant-btn-primary:hover {
        background-color: ${adjustBrightness(color, 10)} !important;
        border-color: ${adjustBrightness(color, 10)} !important;
      }
      .ant-menu-item-selected {
        background-color: ${color}15 !important;
        color: ${color} !important;
      }
      .ant-menu-item-selected .ant-menu-item-icon {
        color: ${color} !important;
      }
      .ant-pagination-item-active {
        background-color: ${color} !important;
        border-color: ${color} !important;
      }
      .ant-switch-checked {
        background-color: ${color} !important;
      }
      .ant-tabs-tab-active .ant-tabs-tab-btn {
        color: ${color} !important;
      }
      .ant-tabs-ink-bar {
        background-color: ${color} !important;
      }
    `
    
    // 移除旧的主题样式
    const oldTheme = document.getElementById('theme-style')
    if (oldTheme) {
      oldTheme.remove()
    }
    
    styleElement.id = 'theme-style'
    document.head.appendChild(styleElement)
  }

  // 应用语言设置
  const applyLanguage = (language) => {
    // 这里可以设置 i18n 的语言
    localStorage.setItem('language', language)
    
    // 如果有 i18n 实例，可以这样设置
    // import { i18n } from '@/i18n'
    // i18n.global.locale = language
  }

  // 保存系统设置
  const saveSystemSettings = async (systemSettings) => {
    try {
      const response = await api.saveSystemSettings(systemSettings)
      if (response.success) {
        settings.value.system = { ...systemSettings }
        
        // 更新页面标题
        document.title = systemSettings.system_name
        
        message.success('系统设置保存成功')
        return true
      }
    } catch (error) {
      message.error('保存失败')
      return false
    }
  }

  // 保存界面设置
  const saveUISettings = async (uiSettings) => {
    try {
      const response = await api.saveUISettings(uiSettings)
      if (response.success) {
        settings.value.ui = { ...uiSettings }
        
        // 立即应用新设置
        applyUISettings()
        
        message.success('界面设置保存成功')
        return true
      }
    } catch (error) {
      message.error('保存失败')
      return false
    }
  }

  // 保存安全设置
  const saveSecuritySettings = async (securitySettings) => {
    try {
      const response = await api.saveSecuritySettings(securitySettings)
      if (response.success) {
        settings.value.security = { ...securitySettings }
        message.success('安全设置保存成功')
        return true
      }
    } catch (error) {
      message.error('保存失败')
      return false
    }
  }

  return {
    settings,
    loading,
    loadSettings,
    applyUISettings,
    saveSystemSettings,
    saveUISettings,
    saveSecuritySettings
  }
})

// 辅助函数：调整颜色亮度
function adjustBrightness(color, percent) {
  const num = parseInt(color.replace("#", ""), 16)
  const amt = Math.round(2.55 * percent)
  const R = (num >> 16) + amt
  const G = (num >> 8 & 0x00FF) + amt
  const B = (num & 0x0000FF) + amt
  return "#" + (0x1000000 + (R < 255 ? R < 1 ? 0 : R : 255) * 0x10000 +
    (G < 255 ? G < 1 ? 0 : G : 255) * 0x100 +
    (B < 255 ? B < 1 ? 0 : B : 255)).toString(16).slice(1)
} 