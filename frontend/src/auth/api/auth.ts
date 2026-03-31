import { api } from '@/shared/api'
import type { AuthModeResponse, LoginToken } from '@/shared/types'

export function fetchAuthMode(baseUrl?: string) {
  return api<AuthModeResponse>('/api/auth/mode', {
    baseURL: baseUrl,
  })
}

export function login(username: string, password: string, baseUrl?: string) {
  return api<LoginToken>('/api/auth/login', {
    method: 'POST',
    baseURL: baseUrl,
    body: { username, password },
  })
}
