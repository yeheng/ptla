import { useAuthStore } from '@/stores/auth'
import type { RouteRecordRaw } from 'vue-router'
import { createRouter, createWebHistory } from 'vue-router'
import MainLayout from '../layouts/MainLayout.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/dashboard'
  },
  {
    path: '/auth',
    component: MainLayout,
    children: [
      {
        path: 'login',
        component: () => import('../views/auth/LoginView.vue')
      },
      {
        path: 'register',
        component: () => import('../views/auth/RegisterView.vue')
      }
    ]
  },
  {
    path: '/',
    component: MainLayout,
    meta: { requiresAuth: true },
    children: [
      {
        path: 'dashboard',
        component: () => import('../views/dashboard/DashboardView.vue')
      },
      {
        path: 'tasks',
        component: () => import('../views/tasks/TasksView.vue')
      },
      {
        path: 'settings',
        component: () => import('../views/settings/SettingsView.vue')
      }
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth)
  const authStore = useAuthStore()

  if (requiresAuth && !authStore.isAuthenticated) {
    next('/auth/login')
  } else {
    next()
  }
})

export default router 