<script setup lang="ts">
import { ref } from 'vue'
import { NyxActionItem, NyxFormField, NyxInput } from 'nyx-kit/components'
import { NyxTheme } from 'nyx-kit/types'
import { NyxKit } from 'nyx-kit'
import { fetchAuthMode } from '@/auth/api'
import { useAuth } from '@/auth/composables'
import { api } from '@/shared/api'
import type { ServerMetadata } from '@/shared/types'

const auth = useAuth()

const serverMetadata = ref<ServerMetadata | null>(null)
const authModeLabel = ref<string>('local')

const syncing = ref(false)
const syncResult = ref<{ homes_scanned: number; vaults_fixed: number; notes_fixed: number } | null>(null)

const removingNotes = ref(false)
const removingVaults = ref(false)
const removingHomes = ref(false)
const removingServerVaults = ref(false)

void (async () => {
  try {
    const [server, authMode] = await Promise.all([
      api<ServerMetadata>('/api/server'),
      fetchAuthMode(),
    ])
    serverMetadata.value = server
    authModeLabel.value = authMode.mode
  } catch {
    serverMetadata.value = null
  }
})()

function signOut() {
  auth.logout()
  window.location.reload()
}

async function syncHomes() {
  if (syncing.value) return
  const result = await NyxKit.confirm({
    title: 'Sync all homes',
    message: 'This will scan all home directories and fix author_id mismatches in vaults and notes. Continue?',
    confirmText: 'Sync',
    theme: NyxTheme.Warning,
  })
  if (!result.isSuccess) return
  syncing.value = true
  syncResult.value = null
  try {
    syncResult.value = await api('/api/admin/sync-homes', { method: 'POST' })
  } finally {
    syncing.value = false
  }
}

async function removeAllNotes() {
  if (removingNotes.value) return
  const result = await NyxKit.confirm({
    title: 'Remove all notes',
    message: 'Delete ALL notes across all vaults? This cannot be undone.',
    confirmText: 'Remove Notes',
    theme: NyxTheme.Danger,
  })
  if (!result.isSuccess) return
  const doubleCheck = await NyxKit.confirm({
    title: 'Are you absolutely sure?',
    message: 'This will permanently delete every note.',
    confirmText: 'Yes, delete everything',
    theme: NyxTheme.Danger,
  })
  if (!doubleCheck.isSuccess) return
  removingNotes.value = true
  try {
    await api('/api/admin/remove-all-notes', { method: 'POST' })
    window.location.reload()
  } finally {
    removingNotes.value = false
  }
}

async function removeAllVaults() {
  if (removingVaults.value) return
  const result = await NyxKit.confirm({
    title: 'Remove all vaults',
    message: 'Delete ALL vaults and their notes? This cannot be undone.',
    confirmText: 'Remove Vaults',
    theme: NyxTheme.Danger,
  })
  if (!result.isSuccess) return
  const doubleCheck = await NyxKit.confirm({
    title: 'Are you absolutely sure?',
    message: 'This will permanently delete every vault and note.',
    confirmText: 'Yes, delete everything',
    theme: NyxTheme.Danger,
  })
  if (!doubleCheck.isSuccess) return
  removingVaults.value = true
  try {
    await api('/api/admin/remove-all-vaults', { method: 'POST' })
    window.location.reload()
  } finally {
    removingVaults.value = false
  }
}

async function removeAllHomes() {
  if (removingHomes.value) return
  const result = await NyxKit.confirm({
    title: 'Remove all homes',
    message: "Delete ALL home directories (users' personal vaults and notes)? This cannot be undone.",
    confirmText: 'Remove Homes',
    theme: NyxTheme.Danger,
  })
  if (!result.isSuccess) return
  const doubleCheck = await NyxKit.confirm({
    title: 'Are you absolutely sure?',
    message: 'This will permanently delete every home directory.',
    confirmText: 'Yes, delete everything',
    theme: NyxTheme.Danger,
  })
  if (!doubleCheck.isSuccess) return
  removingHomes.value = true
  try {
    await api('/api/admin/remove-all-homes', { method: 'POST' })
    window.location.reload()
  } finally {
    removingHomes.value = false
  }
}

async function removeAllServerVaults() {
  if (removingServerVaults.value) return
  const result = await NyxKit.confirm({
    title: 'Remove all server vaults',
    message: 'Delete ALL server vaults and their notes? This cannot be undone.',
    confirmText: 'Remove Server Vaults',
    theme: NyxTheme.Danger,
  })
  if (!result.isSuccess) return
  const doubleCheck = await NyxKit.confirm({
    title: 'Are you absolutely sure?',
    message: 'This will permanently delete every server vault.',
    confirmText: 'Yes, delete everything',
    theme: NyxTheme.Danger,
  })
  if (!doubleCheck.isSuccess) return
  removingServerVaults.value = true
  try {
    await api('/api/admin/remove-all-server-vaults', { method: 'POST' })
    window.location.reload()
  } finally {
    removingServerVaults.value = false
  }
}
</script>

<template>
  <div class="settings-page">
    <div class="settings-page__inner">
      <div class="settings-page__content">
        <h2>Server</h2>
        <section class="settings-section">
          <div class="settings-section__settings">
            <NyxFormField label="Server Name">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="serverMetadata?.name ?? 'Main Server'" disabled />
              </template>
            </NyxFormField>

            <NyxFormField label="Root Path">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="serverMetadata?.root_path ?? ''" disabled />
              </template>
            </NyxFormField>

            <NyxFormField label="Server Slug">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="serverMetadata?.slug ?? ''" disabled />
              </template>
            </NyxFormField>

            <NyxFormField label="Auth Mode">
              <template #default="{ id }">
                <NyxInput :id="id" :model-value="authModeLabel" disabled />
              </template>
            </NyxFormField>
          </div>
        </section>

        <h2>Session</h2>
        <section v-if="auth.isAuthenticated.value" class="settings-section">
          <NyxActionItem
            title="Sign out"
            :theme="NyxTheme.Primary"
            action="Sign Out"
            @click="signOut"
          >Sign out of your current session.</NyxActionItem>
        </section>

        <h2>Sync</h2>
        <section class="settings-section">
          <NyxActionItem
            title="Sync all homes"
            :theme="NyxTheme.Warning"
            action="Sync"
            :loading="syncing"
            @click="syncHomes"
          >Scan all home directories and fix author_id mismatches between the filesystem and note frontmatter.</NyxActionItem>
          <div v-if="syncResult" class="sync-result">
            <span>{{ syncResult.homes_scanned }} homes scanned</span>
            <span>{{ syncResult.vaults_fixed }} vaults fixed</span>
            <span>{{ syncResult.notes_fixed }} notes fixed</span>
          </div>
        </section>

        <h2 class="danger-title">Danger Zone</h2>
        <section class="settings-section settings-section--danger">
          <p class="settings-section__desc">
            These actions are destructive and cannot be undone.
          </p>

          <div class="danger-grid">
            <NyxActionItem
              title="Remove all notes"
              :theme="NyxTheme.Danger"
              action="Remove Notes"
              @click="removeAllNotes"
            >Delete every note across all vaults. Vaults remain.</NyxActionItem>

            <NyxActionItem
              title="Remove all vaults"
              :theme="NyxTheme.Danger"
              action="Remove Vaults"
              @click="removeAllVaults"
            >Delete every vault and all notes inside them.</NyxActionItem>

            <NyxActionItem
              title="Remove all homes"
              :theme="NyxTheme.Danger"
              action="Remove Homes"
              @click="removeAllHomes"
            >Delete all user home directories (personal vaults and notes).</NyxActionItem>

            <NyxActionItem
              title="Remove all server vaults"
              :theme="NyxTheme.Danger"
              action="Remove Server Vaults"
              @click="removeAllServerVaults"
            >Delete all shared server vaults and their notes.</NyxActionItem>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  min-height: 100vh;
  padding: 2rem 1rem;
  overflow-y: auto;
}

.settings-page__inner {
  max-width: 800px;
  margin: 0 auto;
}

.settings-page__header {
  margin-bottom: 2rem;
}

.settings-page__title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
}

.settings-page__content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.settings-section {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-lg);
  padding: 1.25rem;
}

.settings-section--danger {
  border: 1px solid rgba(var(--nyx-rgb-danger), 0.3);
}

.settings-section__settings {
  display: grid;
  gap: 1rem;
}

.settings-section__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.settings-section__header h2 {
  font-size: 0.875rem;
  font-weight: 600;
  margin: 0;
}

.settings-section__actions {
  display: flex;
  gap: 0.5rem;
}

.settings-section__desc {
  font-size: 0.75rem;
  color: var(--nyx-c-text-3);
  margin: 0 0 1rem;
  line-height: 1.5;
}

.danger-title {
  color: var(--nyx-c-danger);
}

.nyx-action-item + .nyx-action-item {
  margin-top: 0.75rem;
}

.sync-result {
  display: flex;
  gap: 1rem;
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: var(--nyx-c-text-3);
}

</style>
