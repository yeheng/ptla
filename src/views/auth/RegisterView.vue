<template>
  <div class="flex min-h-screen items-center justify-center bg-gray-50">
    <div class="w-full max-w-md space-y-8 rounded-lg bg-white p-6 shadow-md">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
          {{ t('auth.register.title') }}
        </h2>
      </div>
      <form class="mt-8 space-y-6" @submit.prevent="handleSubmit">
        <div class="space-y-4 rounded-md shadow-sm">
          <!-- Username field -->
          <div>
            <label for="username" class="block text-sm font-medium text-gray-700">
              {{ t('auth.register.username') }}
            </label>
            <input
              id="username"
              v-model="form.username"
              name="username"
              type="text"
              required
              class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              :placeholder="t('auth.register.usernamePlaceholder')"
            />
          </div>

          <!-- Email field -->
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700">
              {{ t('auth.register.email') }}
            </label>
            <input
              id="email"
              v-model="form.email"
              name="email"
              type="email"
              required
              class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              :placeholder="t('auth.register.emailPlaceholder')"
            />
          </div>

          <!-- Password field -->
          <div>
            <label for="password" class="block text-sm font-medium text-gray-700">
              {{ t('auth.register.password') }}
            </label>
            <input
              id="password"
              v-model="form.password"
              name="password"
              type="password"
              required
              class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              :placeholder="t('auth.register.passwordPlaceholder')"
            />
          </div>

          <!-- Confirm Password field -->
          <div>
            <label for="confirmPassword" class="block text-sm font-medium text-gray-700">
              {{ t('auth.register.confirmPassword') }}
            </label>
            <input
              id="confirmPassword"
              v-model="form.confirmPassword"
              name="confirmPassword"
              type="password"
              required
              class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              :placeholder="t('auth.register.confirmPasswordPlaceholder')"
            />
          </div>
        </div>

        <div>
          <button
            type="submit"
            :disabled="loading"
            class="group relative flex w-full justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            <span class="absolute inset-y-0 left-0 flex items-center pl-3">
              <UserIcon v-if="!loading" class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400" />
              <svg
                v-else
                class="h-5 w-5 animate-spin text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
              </svg>
            </span>
            {{ loading ? t('auth.register.registering') : t('auth.register.submit') }}
          </button>
        </div>

        <!-- Error message -->
        <div v-if="error" class="mt-2 text-center text-sm text-red-600">
          {{ error }}
        </div>

        <!-- Login link -->
        <div class="text-center text-sm">
          {{ t('auth.register.haveAccount') }}
          <router-link to="/login" class="font-medium text-indigo-600 hover:text-indigo-500">
            {{ t('auth.register.loginLink') }}
          </router-link>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { UserIcon } from '@heroicons/vue/24/outline'
import { reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const router = useRouter()
const { t } = useI18n()
const authStore = useAuthStore()

const loading = ref(false)
const error = ref('')

const form = reactive({
  username: '',
  email: '',
  password: '',
  confirmPassword: ''
})

const handleSubmit = async () => {
  // Reset error
  error.value = ''

  // Validate passwords match
  if (form.password !== form.confirmPassword) {
    error.value = t('auth.register.passwordMismatch')
    return
  }

  try {
    loading.value = true
    await authStore.register({
      username: form.username,
      email: form.email,
      password: form.password
    })
    // Registration successful, redirect to login
    router.push('/login')
  } catch (e) {
    error.value = t('auth.register.error')
  } finally {
    loading.value = false
  }
}
</script> 