<template>
  <div class="container relative min-h-screen flex-col items-center justify-center grid lg:max-w-none lg:grid-cols-2 lg:px-0">
    <div class="relative hidden h-full flex-col bg-muted p-10 text-white dark:border-r lg:flex">
      <div class="absolute inset-0 bg-primary" />
      <div class="relative z-20 flex items-center text-lg font-medium">
        <span class="text-primary-foreground">Todo App</span>
      </div>
      <div class="relative z-20 mt-auto">
        <blockquote class="space-y-2">
          <p class="text-lg text-primary-foreground">
            "这是一个简单而强大的任务管理应用，帮助你更好地组织工作和生活。"
          </p>
        </blockquote>
      </div>
    </div>
    <div class="lg:p-8">
      <div class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
        <RouterView />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script> 