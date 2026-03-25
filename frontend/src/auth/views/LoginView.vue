<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuth } from '@/auth/composables'
import { NyxButton, NyxInput, NyxForm, NyxFormField } from 'nyx-kit/components'
import { NyxInputType } from 'nyx-kit/types'

const router = useRouter()
const route = useRoute()
const { authMode, oidcIssuer, login } = useAuth()

const email = ref('')
const password = ref('')
const error = ref<string | null>(null)
const loading = ref(false)

async function handleLogin() {
  error.value = null
  loading.value = true
  try {
    await login(email.value, password.value)
    const redirect = (route.query.redirect as string) ?? '/'
    router.push(redirect)
  } catch {
    error.value = 'Invalid credentials'
  } finally {
    loading.value = false
  }
}

function handleOidc() {
  // OIDC redirect — issuer URL comes from server discovery
  if (oidcIssuer.value) {
    window.location.href = oidcIssuer.value
  }
}
</script>

<template>
  <div class="login">
    <h1>Nyx Notes</h1>

    <NyxForm v-if="authMode === 'secret_key'" @submit="handleLogin">
      <NyxFormField label="Email">
        <NyxInput v-model="email" :type="NyxInputType.Email" placeholder="you@example.com" required />
      </NyxFormField>
      <NyxFormField label="Password">
        <NyxInput v-model="password" :type="NyxInputType.Password" placeholder="••••••••" required />
      </NyxFormField>
      <p v-if="error" class="login__error">{{ error }}</p>
      <NyxButton type="submit" :disabled="loading">
        {{ loading ? 'Signing in…' : 'Sign in' }}
      </NyxButton>
    </NyxForm>

    <div v-else-if="authMode === 'oidc'">
      <NyxButton @click="handleOidc">Sign in with SSO</NyxButton>
    </div>
  </div>
</template>

<style scoped>
.login {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  gap: 1.5rem;
}

.login__error {
  color: var(--nyx-color-danger, #e53e3e);
  font-size: 0.875rem;
}
</style>
