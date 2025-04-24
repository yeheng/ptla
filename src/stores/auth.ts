import { defineStore } from 'pinia'
import { ref } from 'vue'

interface User {
  id: number
  username: string
  email: string
  role: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(null)
  const isAuthenticated = ref(false)

  function setUser(userData: User | null) {
    user.value = userData
    isAuthenticated.value = !!userData
  }

  function setToken(newToken: string | null) {
    token.value = newToken
    localStorage.setItem('token', newToken || '')
  }

  function logout() {
    user.value = null
    token.value = null
    isAuthenticated.value = false
    localStorage.removeItem('token')
  }

  // 初始化时检查本地存储的token
  const storedToken = localStorage.getItem('token')
  if (storedToken) {
    token.value = storedToken
    // TODO: 验证token有效性并获取用户信息
  }

  return {
    user,
    token,
    isAuthenticated,
    setUser,
    setToken,
    logout
  }
}) 