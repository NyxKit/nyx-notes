import { api } from '@/shared/api'
import type { AuthModeResponse, LoginToken } from '@/shared/types'

export function fetchAuthMode(baseUrl?: string) {
  return api<AuthModeResponse>('/api/auth/mode', {
    baseURL: baseUrl,
  })
}

export function fetchInitialized(baseUrl?: string) {
  return api<{ initialized: boolean }>('/api/auth/initialized', {
    baseURL: baseUrl,
  })
}

export interface SetupInput {
  username: string
  email: string
  display_name: string
  password: string
  confirm_password: string
}

export function setup(input: SetupInput, baseUrl?: string) {
  const { confirm_password, ...body } = input
  return api<LoginToken>('/api/auth/setup', {
    method: 'POST',
    baseURL: baseUrl,
    body,
  })
}

export function login(username: string, password: string, baseUrl?: string) {
  return api<LoginToken>('/api/auth/login', {
    method: 'POST',
    baseURL: baseUrl,
    body: { username, password },
  })
}
