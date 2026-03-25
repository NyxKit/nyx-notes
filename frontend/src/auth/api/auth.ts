import { api } from '@/shared/api'
import type { AuthModeResponse, LoginToken } from '@/shared/types'

export function fetchAuthMode() {
  return api<AuthModeResponse>('/api/auth/mode')
}

export function login(email: string, password: string) {
  return api<LoginToken>('/api/auth/login', {
    method: 'POST',
    body: { email, password },
  })
}
