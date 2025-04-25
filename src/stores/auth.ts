import { defineStore } from 'pinia'
import { ref } from 'vue'

interface User {
  id: number
  username: string
  email: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isAuthenticated = ref(false)

  function setUser(newUser: User | null) {
    user.value = newUser
    isAuthenticated.value = !!newUser
  }

  function login(credentials: { username: string; password: string }) {
    // TODO: 实现实际的登录逻辑
    setUser({
      id: 1,
      username: credentials.username,
      email: `${credentials.username}@example.com`
    })
  }

  function register(userData: { username: string; email: string; password: string }) {
    // TODO: 实现实际的注册逻辑
    setUser({
      id: 1,
      username: userData.username,
      email: userData.email
    })
  }

  function logout() {
    setUser(null)
  }

  return {
    user,
    isAuthenticated,
    login,
    register,
    logout
  }
}) 