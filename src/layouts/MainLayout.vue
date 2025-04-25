<template>
  <div class="min-h-screen bg-background">
    <!-- 顶部导航栏 -->
    <header class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div class="container flex h-14 items-center">
        <div class="mr-4 flex">
          <a class="mr-6 flex items-center space-x-2" href="/">
            <span class="font-bold">Todo App</span>
          </a>
        </div>
        <div class="flex flex-1 items-center justify-between space-x-2 md:justify-end">
          <nav class="flex items-center">
            <RouterLink
              v-for="item in navigation"
              :key="item.href"
              :to="item.href"
              class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2"
              :class="{ 'bg-accent': isCurrentRoute(item.href) }"
            >
              {{ item.name }}
            </RouterLink>
          </nav>
        </div>
      </div>
    </header>

    <!-- 主要内容区域 -->
    <main class="container py-6">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'

const navigation = [
  { name: '仪表盘', href: '/dashboard' },
  { name: '任务', href: '/tasks' },
  { name: '设置', href: '/settings' }
]

const route = useRoute()
const isCurrentRoute = (href: string) => route.path === href
</script> 