import { api } from '@/shared/api'
import type { CreateUserRequest, ManagedUserSummary, UpdateUserRequest } from '@/shared/types'

export function fetchUsers() {
  return api<ManagedUserSummary[]>('/api/users')
}

export function createUser(body: CreateUserRequest) {
  return api<ManagedUserSummary>('/api/users', {
    method: 'POST',
    body,
  })
}

export function updateUser(userId: string, body: UpdateUserRequest) {
  return api<ManagedUserSummary>(`/api/users/${userId}`, {
    method: 'PATCH',
    body,
  })
}

export function deleteUser(userId: string) {
  return api(`/api/users/${userId}`, {
    method: 'DELETE',
  })
}
