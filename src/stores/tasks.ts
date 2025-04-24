import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

interface Task {
  id: number
  title: string
  description?: string
  status: 'pending' | 'in_progress' | 'completed' | 'cancelled'
  priority: 0 | 1 | 2
  dueDate?: Date
  completedAt?: Date
  parentTaskId?: number
  position: number
  tags: string[]
  metadata: Record<string, any>
}

interface TaskFilter {
  status?: Task['status']
  priority?: Task['priority']
  search?: string
  tags?: string[]
}

interface TaskSorting {
  field: keyof Task
  order: 'asc' | 'desc'
}

export const useTaskStore = defineStore('tasks', () => {
  const tasks = ref<Task[]>([])
  const currentTask = ref<Task | null>(null)
  const filters = ref<TaskFilter>({})
  const sorting = ref<TaskSorting>({
    field: 'position',
    order: 'asc'
  })

  // 计算属性：过滤后的任务列表
  const filteredTasks = computed(() => {
    let result = [...tasks.value]

    // 应用过滤器
    if (filters.value.status) {
      result = result.filter(task => task.status === filters.value.status)
    }
    if (filters.value.priority !== undefined) {
      result = result.filter(task => task.priority === filters.value.priority)
    }
    if (filters.value.search) {
      const searchLower = filters.value.search.toLowerCase()
      result = result.filter(task => 
        task.title.toLowerCase().includes(searchLower) ||
        task.description?.toLowerCase().includes(searchLower)
      )
    }
    if (filters.value.tags?.length) {
      result = result.filter(task => 
        filters.value.tags?.some(tag => task.tags.includes(tag))
      )
    }

    // 应用排序
    result.sort((a, b) => {
      const aValue = a[sorting.value.field]
      const bValue = b[sorting.value.field]
      const order = sorting.value.order === 'asc' ? 1 : -1

      // Handle undefined values
      if (aValue === undefined && bValue === undefined) return 0
      if (aValue === undefined) return order
      if (bValue === undefined) return -order

      if (aValue < bValue) return -1 * order
      if (aValue > bValue) return 1 * order
      return 0
    })

    return result
  })

  // 方法
  function setTasks(newTasks: Task[]) {
    tasks.value = newTasks
  }

  function addTask(task: Omit<Task, 'id'> & { id?: number }) {
    const newTask = {
      ...task,
      id: task.id ?? Math.max(0, ...tasks.value.map(t => t.id)) + 1
    }
    tasks.value.push(newTask)
  }

  function updateTask(taskId: number, updates: Partial<Task>) {
    const index = tasks.value.findIndex(t => t.id === taskId)
    if (index !== -1) {
      tasks.value[index] = { ...tasks.value[index], ...updates }
    }
  }

  function deleteTask(taskId: number) {
    tasks.value = tasks.value.filter(t => t.id !== taskId)
  }

  function setCurrentTask(task: Task | null) {
    currentTask.value = task
  }

  function setFilters(newFilters: TaskFilter) {
    filters.value = newFilters
  }

  function setSorting(newSorting: TaskSorting) {
    sorting.value = newSorting
  }

  return {
    tasks,
    currentTask,
    filters,
    sorting,
    filteredTasks,
    setTasks,
    addTask,
    updateTask,
    deleteTask,
    setCurrentTask,
    setFilters,
    setSorting
  }
}) 