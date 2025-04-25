<template>
  <div class="flex flex-col space-y-2 text-center">
    <h1 class="text-2xl font-semibold tracking-tight">登录账户</h1>
    <p class="text-sm text-muted-foreground">
      输入您的账户信息以登录
    </p>
  </div>
  <div class="grid gap-6">
    <form @submit.prevent="handleSubmit">
      <div class="grid gap-4">
        <div class="grid gap-2">
          <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="username">
            用户名
          </label>
          <input
            id="username"
            v-model="form.username"
            class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="输入用户名"
            required
          />
        </div>
        <div class="grid gap-2">
          <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="password">
            密码
          </label>
          <input
            id="password"
            v-model="form.password"
            type="password"
            class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="输入密码"
            required
          />
        </div>
        <button
          class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
          type="submit"
        >
          登录
        </button>
      </div>
    </form>
    <div class="relative">
      <div class="absolute inset-0 flex items-center">
        <span class="w-full border-t" />
      </div>
      <div class="relative flex justify-center text-xs uppercase">
        <span class="bg-background px-2 text-muted-foreground">
          或者
        </span>
      </div>
    </div>
    <RouterLink
      to="/auth/register"
      class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2"
    >
      创建新账户
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { reactive } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const authStore = useAuthStore()

const form = reactive({
  username: '',
  password: ''
})

async function handleSubmit() {
  try {
    await authStore.login(form)
    router.push('/dashboard')
  } catch (error) {
    console.error('登录失败:', error)
  }
}
</script> 