<template>
  <div class="flex flex-col space-y-2 text-center">
    <h1 class="text-2xl font-semibold tracking-tight">{{ t('auth.login.title') }}</h1>
    <p class="text-sm text-muted-foreground">
      {{ t('auth.login.description') }}
    </p>
  </div>
  <div class="grid gap-6 p-6">
    <form @submit.prevent="handleSubmit">
      <div class="grid gap-4">
        <div class="grid gap-2">
          <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            for="username">
            {{ t('auth.login.username') }}
          </label>
          <input id="username" v-model="form.username" autocomplete="usernam"
            class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            :placeholder="t('auth.login.usernamePlaceholder')" required />
        </div>
        <div class="grid gap-2">
          <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            for="password">
            {{ t('auth.login.password') }}
          </label>
          <input id="password" v-model="form.password" type="password" autocomplete="current-password"
            class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
            :placeholder="t('auth.login.passwordPlaceholder')" required />
        </div>
        <button
          class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
          type="submit" :disabled="loading">
          {{ loading ? t('auth.login.loggingIn') : t('auth.login.submit') }}
        </button>
      </div>
    </form>
    <div class="relative">
      <div class="absolute inset-0 flex items-center">
        <span class="w-full border-t" />
      </div>
      <div class="relative flex justify-center text-xs uppercase">
        <span class="bg-background px-2 text-muted-foreground">
          {{ t('auth.login.or') }}
        </span>
      </div>
    </div>
    <RouterLink to="/auth/register"
      class="inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2">
      {{ t('auth.login.createAccount') }}
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
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
  password: ''
})

async function handleSubmit() {
  try {
    loading.value = true
    await authStore.login(form)
    router.push('/dashboard')
  } catch (error) {
    console.error(t('auth.login.error'), error)
  } finally {
    loading.value = false
  }
}
</script>