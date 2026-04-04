<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { NyxButton } from 'nyx-kit/components'
import { useAuth } from '@/auth/composables'
import { CreateEditUser, UsersTable } from '@/users/components'
import { useUsers } from '@/users/composables'
import type { ManagedUserSummary } from '@/shared/types'

const auth = useAuth()
const { users, loading, saving, deleting, error, load, create, update, remove } = useUsers()

const showModal = ref(false)
const editingUser = ref<ManagedUserSummary | null>(null)
const canTeleportHeaderActions = ref(false)

const currentUserId = computed(() => auth.serverMetadata.value?.current_user_id ?? null)
const canManageUsers = computed(() => auth.authMode.value === 'secret_key' && auth.serverMetadata.value?.role === 'admin')

onMounted(() => {
  canTeleportHeaderActions.value = Boolean(document.getElementById('layout-header-actions'))
})

watch(
  canManageUsers,
  (allowed) => {
    if (allowed) {
      void load()
    }
  },
  { immediate: true }
)

function openCreate() {
  editingUser.value = null
  showModal.value = true
}

function openEdit(user: ManagedUserSummary) {
  editingUser.value = user
  showModal.value = true
}

async function submit(payload: { username: string; email: string; display_name: string; role: 'admin' | 'user'; password?: string }) {
  if (editingUser.value) {
    await update(editingUser.value.id, {
      email: payload.email,
      display_name: payload.display_name,
      role: payload.role,
      password: payload.password,
    })
  } else {
    await create({
      username: payload.username,
      email: payload.email,
      display_name: payload.display_name,
      role: payload.role,
      password: payload.password ?? '',
    })
  }

  showModal.value = false
  editingUser.value = null
}

async function confirmDelete(user: ManagedUserSummary) {
  if (!window.confirm(`Delete ${user.username}?`)) return
  await remove(user.id)
}
</script>

<template>
  <section class="users-view">
    <Teleport v-if="canTeleportHeaderActions" to="#layout-header-actions">
      <NyxButton v-if="canManageUsers" :gradient="true" @click="openCreate">Add user</NyxButton>
    </Teleport>

    <header class="users-view__header">
      <div>
        <h1>Users</h1>
        <p>Manage server users for this secret_key server.</p>
      </div>
      <NyxButton v-if="canManageUsers && !canTeleportHeaderActions" :gradient="true" @click="openCreate">Add user</NyxButton>
    </header>

    <div v-if="!canManageUsers" class="users-view__empty">
      <p>User management is available only to administrators in secret_key mode.</p>
    </div>

    <div v-else-if="loading" class="users-view__empty">
      <p>Loading users…</p>
    </div>

    <div v-else-if="error" class="users-view__empty">
      <p>{{ error }}</p>
      <NyxButton @click="load">Retry</NyxButton>
    </div>

    <div v-else-if="users.length === 0" class="users-view__empty">
      <p>No users yet.</p>
      <NyxButton :gradient="true" @click="openCreate">Add user</NyxButton>
    </div>

    <UsersTable
      v-else
      :users="users"
      :deleting-id="deleting"
      @edit="openEdit"
      @delete="confirmDelete"
    />

    <CreateEditUser
      v-model:open="showModal"
      :user="editingUser"
      :saving="saving"
      :current-user-id="currentUserId"
      @submit="submit"
    />
  </section>
</template>

<style scoped>
.users-view {
  padding: 1.5rem;
  display: grid;
  gap: 1rem;
}

.users-view__header h1 {
  margin: 0;
}

.users-view__header p {
  margin: 0.25rem 0 0;
  color: var(--nyx-c-text-2);
}

.users-view__empty {
  display: grid;
  gap: 1rem;
  justify-items: start;
}
</style>
