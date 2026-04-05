import { computed, ref } from 'vue'
import { createUser, deleteUser, fetchUsers, updateUser } from '@/users/api'
import type { CreateUserRequest, ManagedUserSummary, UpdateUserRequest } from '@/shared/types'

const users = ref<ManagedUserSummary[]>([])
const loading = ref(false)
const saving = ref(false)
const deleting = ref<string | null>(null)
const error = ref<string | null>(null)

export function useUsers() {
  const sortedUsers = computed(() => [...users.value].sort((a, b) => a.username.localeCompare(b.username)))

  async function load() {
    loading.value = true
    error.value = null
    try {
      users.value = await fetchUsers()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load users'
    } finally {
      loading.value = false
    }
  }

  async function create(body: CreateUserRequest) {
    saving.value = true
    error.value = null
    try {
      const created = await createUser(body)
      users.value = [...users.value, created]
      return created
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to create user'
      error.value = message
      throw new Error(message)
    } finally {
      saving.value = false
    }
  }

  async function update(userId: string, body: UpdateUserRequest) {
    saving.value = true
    error.value = null
    try {
      const updated = await updateUser(userId, body)
      users.value = users.value.map(user => user.id === userId ? updated : user)
      return updated
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to update user'
      error.value = message
      throw new Error(message)
    } finally {
      saving.value = false
    }
  }

  async function remove(userId: string) {
    deleting.value = userId
    error.value = null
    try {
      await deleteUser(userId)
      users.value = users.value.filter(user => user.id !== userId)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to delete user'
      error.value = message
      throw new Error(message)
    } finally {
      deleting.value = null
    }
  }

  return {
    users: sortedUsers,
    loading,
    saving,
    deleting,
    error,
    load,
    create,
    update,
    remove,
  }
}
