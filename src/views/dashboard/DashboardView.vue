<template>
  <div class="flex flex-col gap-4">
    <div class="flex items-center justify-between space-y-2">
      <h2 class="text-3xl font-bold tracking-tight">{{ t('dashboard.title') }}</h2>
    </div>
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
        <div class="flex flex-row items-center justify-between space-y-0 pb-2">
          <h3 class="tracking-tight text-sm font-medium">{{ t('dashboard.stats.totalTasks') }}</h3>
        </div>
        <div class="text-2xl font-bold">{{ totalTasks }}</div>
      </div>
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
        <div class="flex flex-row items-center justify-between space-y-0 pb-2">
          <h3 class="tracking-tight text-sm font-medium">{{ t('dashboard.stats.pendingTasks') }}</h3>
        </div>
        <div class="text-2xl font-bold">{{ pendingTasks }}</div>
      </div>
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
        <div class="flex flex-row items-center justify-between space-y-0 pb-2">
          <h3 class="tracking-tight text-sm font-medium">{{ t('dashboard.stats.inProgressTasks') }}</h3>
        </div>
        <div class="text-2xl font-bold">{{ inProgressTasks }}</div>
      </div>
      <div class="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
        <div class="flex flex-row items-center justify-between space-y-0 pb-2">
          <h3 class="tracking-tight text-sm font-medium">{{ t('dashboard.stats.completedTasks') }}</h3>
        </div>
        <div class="text-2xl font-bold">{{ completedTasks }}</div>
      </div>
    </div>
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
      <div class="col-span-4">
        <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
          <div class="p-6">
            <h3 class="font-semibold leading-none tracking-tight">{{ t('dashboard.recentTasks.title') }}</h3>
            <p class="text-sm text-muted-foreground">
              {{ t('dashboard.recentTasks.description') }}
            </p>
          </div>
          <div class="p-6 pt-0">
            <div class="space-y-4">
              <div
                v-for="task in recentTasks"
                :key="task.id"
                class="flex items-center"
              >
                <div class="space-y-1">
                  <p class="text-sm font-medium leading-none">{{ task.title }}</p>
                  <p class="text-sm text-muted-foreground">
                    {{ task.description }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-span-3">
        <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
          <div class="p-6">
            <h3 class="font-semibold leading-none tracking-tight">{{ t('dashboard.taskStats.title') }}</h3>
            <p class="text-sm text-muted-foreground">
              {{ t('dashboard.taskStats.description') }}
            </p>
          </div>
          <div class="p-6 pt-0">
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4">
                  <div class="space-y-1">
                    <p class="text-sm font-medium leading-none">{{ t('status.pending') }}</p>
                  </div>
                </div>
                <div>{{ pendingTasks }}</div>
              </div>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4">
                  <div class="space-y-1">
                    <p class="text-sm font-medium leading-none">{{ t('status.in_progress') }}</p>
                  </div>
                </div>
                <div>{{ inProgressTasks }}</div>
              </div>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-4">
                  <div class="space-y-1">
                    <p class="text-sm font-medium leading-none">{{ t('status.completed') }}</p>
                  </div>
                </div>
                <div>{{ completedTasks }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTaskStore } from '@/stores/tasks'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const taskStore = useTaskStore()

const totalTasks = computed(() => taskStore.tasks.length)
const pendingTasks = computed(() => taskStore.tasks.filter(t => t.status === 'pending').length)
const inProgressTasks = computed(() => taskStore.tasks.filter(t => t.status === 'in_progress').length)
const completedTasks = computed(() => taskStore.tasks.filter(t => t.status === 'completed').length)

const recentTasks = computed(() => {
  return [...taskStore.tasks]
    .sort((a, b) => {
      const dateA = a.completedAt || new Date()
      const dateB = b.completedAt || new Date()
      return dateB.getTime() - dateA.getTime()
    })
    .slice(0, 5)
})
</script> 