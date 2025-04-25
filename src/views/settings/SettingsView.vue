<template>
  <div class="flex flex-col gap-4">
    <div class="flex items-center justify-between space-y-2">
      <h2 class="text-3xl font-bold tracking-tight">设置</h2>
    </div>
    <div class="grid gap-4">
      <!-- 个人信息设置 -->
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
        <div class="p-6">
          <h3 class="text-lg font-semibold">个人信息</h3>
          <div class="mt-4 space-y-4">
            <div class="grid gap-2">
              <label class="text-sm font-medium leading-none" for="username">
                用户名
              </label>
              <input
                id="username"
                v-model="userSettings.username"
                class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="true"
              />
            </div>
            <div class="grid gap-2">
              <label class="text-sm font-medium leading-none" for="email">
                电子邮箱
              </label>
              <input
                id="email"
                v-model="userSettings.email"
                class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="true"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- 主题设置 -->
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
        <div class="p-6">
          <h3 class="text-lg font-semibold">主题设置</h3>
          <div class="mt-4 space-y-4">
            <div class="flex items-center space-x-2">
              <label class="text-sm font-medium leading-none" for="theme">
                主题模式
              </label>
              <select
                id="theme"
                v-model="userSettings.theme"
                class="flex h-9 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                @change="updateTheme"
              >
                <option value="light">浅色</option>
                <option value="dark">深色</option>
                <option value="system">跟随系统</option>
              </select>
            </div>
          </div>
        </div>
      </div>

      <!-- 通知设置 -->
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
        <div class="p-6">
          <h3 class="text-lg font-semibold">通知设置</h3>
          <div class="mt-4 space-y-4">
            <div class="flex items-center space-x-2">
              <input
                id="emailNotifications"
                v-model="userSettings.emailNotifications"
                type="checkbox"
                class="h-4 w-4 rounded border-input bg-transparent shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
              />
              <label class="text-sm font-medium leading-none" for="emailNotifications">
                接收邮件通知
              </label>
            </div>
            <div class="flex items-center space-x-2">
              <input
                id="desktopNotifications"
                v-model="userSettings.desktopNotifications"
                type="checkbox"
                class="h-4 w-4 rounded border-input bg-transparent shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
              />
              <label class="text-sm font-medium leading-none" for="desktopNotifications">
                接收桌面通知
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- 保存按钮 -->
      <div class="flex justify-end">
        <button
          @click="saveSettings"
          class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
        >
          保存设置
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { reactive } from 'vue'

const authStore = useAuthStore()

const userSettings = reactive({
  username: authStore.user?.username || '',
  email: authStore.user?.email || '',
  theme: localStorage.getItem('theme') || 'system',
  emailNotifications: true,
  desktopNotifications: false
})

function updateTheme() {
  localStorage.setItem('theme', userSettings.theme)
  if (userSettings.theme === 'dark') {
    document.documentElement.classList.add('dark')
  } else if (userSettings.theme === 'light') {
    document.documentElement.classList.remove('dark')
  } else {
    // 系统主题
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }
}

function saveSettings() {
  // TODO: 实现设置保存逻辑
  alert('设置已保存')
}

// 初始化主题
updateTheme()
</script> 