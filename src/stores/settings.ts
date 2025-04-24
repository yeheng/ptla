import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

interface NotificationSettings {
  enabled: boolean
  sound: boolean
  desktop: boolean
  email: boolean
  dueDateReminder: boolean
  reminderTime: number // 提前提醒时间（分钟）
}

interface Settings {
  theme: 'light' | 'dark' | 'system'
  language: string
  notifications: NotificationSettings
  sidebarCollapsed: boolean
  taskViewMode: 'list' | 'board' | 'calendar'
  taskSortOrder: 'manual' | 'dueDate' | 'priority' | 'title'
  taskGrouping: 'none' | 'status' | 'priority' | 'dueDate'
}

export const useSettingsStore = defineStore('settings', () => {
  // 默认设置
  const defaultSettings: Settings = {
    theme: 'system',
    language: 'zh-CN',
    notifications: {
      enabled: true,
      sound: true,
      desktop: true,
      email: false,
      dueDateReminder: true,
      reminderTime: 30
    },
    sidebarCollapsed: false,
    taskViewMode: 'list',
    taskSortOrder: 'manual',
    taskGrouping: 'none'
  }

  // 从本地存储加载设置
  const loadSettings = (): Settings => {
    const stored = localStorage.getItem('settings')
    if (stored) {
      try {
        return { ...defaultSettings, ...JSON.parse(stored) }
      } catch (e) {
        console.error('Failed to parse stored settings:', e)
      }
    }
    return defaultSettings
  }

  const settings = ref<Settings>(loadSettings())

  // 监听设置变化并保存到本地存储
  watch(settings, (newSettings) => {
    localStorage.setItem('settings', JSON.stringify(newSettings))
  }, { deep: true })

  // 更新设置
  function updateSettings(updates: Partial<Settings>) {
    settings.value = {
      ...settings.value,
      ...updates
    }
  }

  // 更新通知设置
  function updateNotificationSettings(updates: Partial<NotificationSettings>) {
    settings.value.notifications = {
      ...settings.value.notifications,
      ...updates
    }
  }

  // 重置设置
  function resetSettings() {
    settings.value = { ...defaultSettings }
  }

  // 应用主题
  function applyTheme(theme?: Settings['theme']) {
    const currentTheme = theme || settings.value.theme
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches

    let isDark = currentTheme === 'dark' || 
                 (currentTheme === 'system' && prefersDark)

    document.documentElement.classList.toggle('dark', isDark)
  }

  // 初始化主题
  applyTheme()

  // 监听系统主题变化
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    if (settings.value.theme === 'system') {
      applyTheme('system')
    }
  })

  return {
    settings,
    updateSettings,
    updateNotificationSettings,
    resetSettings,
    applyTheme
  }
}) 