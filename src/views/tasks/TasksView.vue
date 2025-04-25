<template>
  <div class="flex flex-col gap-4">
    <div class="flex items-center justify-between space-y-2">
      <h2 class="text-3xl font-bold tracking-tight">任务管理</h2>
      <button
        @click="showAddTaskDialog = true"
        class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
      >
        新建任务
      </button>
    </div>

    <!-- 任务列表 -->
    <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
      <div class="p-0">
        <div class="relative w-full overflow-auto">
          <table class="w-full caption-bottom text-sm">
            <thead class="[&_tr]:border-b">
              <tr class="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  任务
                </th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  状态
                </th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  优先级
                </th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  截止日期
                </th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  操作
                </th>
              </tr>
            </thead>
            <tbody class="[&_tr:last-child]:border-0">
              <tr
                v-for="task in taskStore.filteredTasks"
                :key="task.id"
                class="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted"
              >
                <td class="p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  <div class="flex flex-col">
                    <span class="font-medium">{{ task.title }}</span>
                    <span class="text-sm text-muted-foreground">{{ task.description }}</span>
                  </div>
                </td>
                <td class="p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  <span
                    class="inline-flex items-center rounded-md px-2 py-1 text-xs font-medium ring-1 ring-inset"
                    :class="{
                      'bg-yellow-50 text-yellow-800 ring-yellow-600/20': task.status === 'pending',
                      'bg-blue-50 text-blue-800 ring-blue-600/20': task.status === 'in_progress',
                      'bg-green-50 text-green-800 ring-green-600/20': task.status === 'completed',
                      'bg-red-50 text-red-800 ring-red-600/20': task.status === 'cancelled'
                    }"
                  >
                    {{ getStatusText(task.status) }}
                  </span>
                </td>
                <td class="p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  <span
                    class="inline-flex items-center rounded-md px-2 py-1 text-xs font-medium ring-1 ring-inset"
                    :class="{
                      'bg-gray-50 text-gray-800 ring-gray-600/20': task.priority === 0,
                      'bg-yellow-50 text-yellow-800 ring-yellow-600/20': task.priority === 1,
                      'bg-red-50 text-red-800 ring-red-600/20': task.priority === 2
                    }"
                  >
                    {{ getPriorityText(task.priority) }}
                  </span>
                </td>
                <td class="p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  {{ task.dueDate ? new Date(task.dueDate).toLocaleDateString() : '无' }}
                </td>
                <td class="p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">
                  <div class="flex gap-2">
                    <button
                      @click="editTask(task)"
                      class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-8 w-8"
                    >
                      <span class="sr-only">编辑</span>
                      📝
                    </button>
                    <button
                      @click="deleteTask(task.id)"
                      class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-8 w-8"
                    >
                      <span class="sr-only">删除</span>
                      🗑️
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- 添加/编辑任务对话框 -->
    <div
      v-if="showAddTaskDialog"
      class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm"
    >
      <div class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 sm:rounded-lg">
        <div class="flex flex-col space-y-1.5 text-center sm:text-left">
          <h3 class="text-lg font-semibold leading-none tracking-tight">
            {{ editingTask ? '编辑任务' : '新建任务' }}
          </h3>
          <p class="text-sm text-muted-foreground">
            {{ editingTask ? '修改任务信息' : '填写任务信息' }}
          </p>
        </div>
        <form @submit.prevent="handleSubmit" class="grid gap-4">
          <div class="grid gap-2">
            <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="title">
              标题
            </label>
            <input
              id="title"
              v-model="taskForm.title"
              class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="输入任务标题"
              required
            />
          </div>
          <div class="grid gap-2">
            <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="description">
              描述
            </label>
            <textarea
              id="description"
              v-model="taskForm.description"
              class="flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="输入任务描述"
            />
          </div>
          <div class="grid gap-2">
            <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="status">
              状态
            </label>
            <select
              id="status"
              v-model="taskForm.status"
              class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option value="pending">待处理</option>
              <option value="in_progress">进行中</option>
              <option value="completed">已完成</option>
              <option value="cancelled">已取消</option>
            </select>
          </div>
          <div class="grid gap-2">
            <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="priority">
              优先级
            </label>
            <select
              id="priority"
              v-model="taskForm.priority"
              class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option :value="0">低</option>
              <option :value="1">中</option>
              <option :value="2">高</option>
            </select>
          </div>
          <div class="grid gap-2">
            <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70" for="dueDate">
              截止日期
            </label>
            <input
              id="dueDate"
              v-model="taskForm.dueDate"
              type="date"
              class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            />
          </div>
          <div class="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2">
            <button
              type="button"
              @click="showAddTaskDialog = false"
              class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2 mt-2 sm:mt-0"
            >
              取消
            </button>
            <button
              type="submit"
              class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
            >
              {{ editingTask ? '保存' : '创建' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTaskStore } from '@/stores/tasks'
import { reactive, ref } from 'vue'

const taskStore = useTaskStore()
const showAddTaskDialog = ref(false)
const editingTask = ref<number | null>(null)

const taskForm = reactive({
  title: '',
  description: '',
  status: 'pending' as const,
  priority: 0 as 0 | 1 | 2,
  dueDate: ''
})

function resetForm() {
  taskForm.title = ''
  taskForm.description = ''
  taskForm.status = 'pending'
  taskForm.priority = 0
  taskForm.dueDate = ''
  editingTask.value = null
}

function editTask(task: any) {
  editingTask.value = task.id
  taskForm.title = task.title
  taskForm.description = task.description || ''
  taskForm.status = task.status
  taskForm.priority = task.priority
  taskForm.dueDate = task.dueDate ? new Date(task.dueDate).toISOString().split('T')[0] : ''
  showAddTaskDialog.value = true
}

function handleSubmit() {
  const taskData = {
    title: taskForm.title,
    description: taskForm.description,
    status: taskForm.status,
    priority: taskForm.priority,
    dueDate: taskForm.dueDate ? new Date(taskForm.dueDate) : undefined,
    tags: [],
    metadata: {}
  }

  if (editingTask.value) {
    taskStore.updateTask(editingTask.value, taskData)
  } else {
    taskStore.addTask({
      ...taskData,
      position: taskStore.tasks.length
    })
  }

  showAddTaskDialog.value = false
  resetForm()
}

function deleteTask(taskId: number) {
  if (confirm('确定要删除这个任务吗？')) {
    taskStore.deleteTask(taskId)
  }
}

function getStatusText(status: string) {
  const statusMap: Record<string, string> = {
    pending: '待处理',
    in_progress: '进行中',
    completed: '已完成',
    cancelled: '已取消'
  }
  return statusMap[status] || status
}

function getPriorityText(priority: number) {
  const priorityMap: Record<number, string> = {
    0: '低',
    1: '中',
    2: '高'
  }
  return priorityMap[priority] || String(priority)
}
</script> 