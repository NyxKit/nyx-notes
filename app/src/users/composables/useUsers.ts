import { computed, ref } from 'vue'
import type { CreateUserRequest, ManagedUserSummary, UpdateUserRequest } from '@/shared/types'

const users = ref<ManagedUserSummary[]>([])
const loading = ref(false)
const saving = ref(false)
const deleting = ref<string | null>(null)
const error = ref<string | null>(null)

export function useUsers() {
  const sortedUsers = computed(() => [...users.value].sort((a, b) => a.username.localeCompare(b.username)))

  async function load() {
    loading.value = false
  }

  async function create(body: CreateUserRequest) {
    void body
    saving.value = false
    return null as unknown as ManagedUserSummary
  }

  async function update(userId: string, body: UpdateUserRequest) {
    void userId
    void body
    saving.value = false
    return null as unknown as ManagedUserSummary
  }

  async function remove(userId: string) {
    void userId
    deleting.value = null
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
