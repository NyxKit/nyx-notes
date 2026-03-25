import { ref, computed } from 'vue'
import { fetchAuthMode, login as apiLogin } from '@/auth/api'
import { setApiToken } from '@/shared/api'
import type { AuthMode, User } from '@/shared/types'

const authMode = ref<AuthMode | null>(null)
const oidcIssuer = ref<string | null>(null)
const token = ref<string | null>(localStorage.getItem('nyx_token'))
const currentUser = ref<User | null>(null)

// Keep the API client in sync with the stored token on startup
if (token.value) setApiToken(token.value)

export function useAuth() {
  const isAuthenticated = computed(() =>
    authMode.value === 'local' || token.value !== null
  )

  async function discoverMode() {
    const res = await fetchAuthMode()
    authMode.value = res.mode
    oidcIssuer.value = res.issuer ?? null
  }

  async function login(email: string, password: string) {
    const res = await apiLogin(email, password)
    token.value = res.token
    localStorage.setItem('nyx_token', res.token)
    setApiToken(res.token)
  }

  function logout() {
    token.value = null
    currentUser.value = null
    localStorage.removeItem('nyx_token')
    setApiToken(null)
  }

  return {
    authMode,
    oidcIssuer,
    token,
    currentUser,
    isAuthenticated,
    discoverMode,
    login,
    logout,
  }
}
