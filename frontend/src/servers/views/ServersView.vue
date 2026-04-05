<script setup lang="ts">
import { ref } from 'vue'
import { NyxButton, NyxFormField, NyxInput } from 'nyx-kit/components'
import { fetchAuthMode } from '@/auth/api'
import { useAuth } from '@/auth/composables'
import { api } from '@/shared/api'
import type { ServerMetadata } from '@/shared/types'
import { NyxTheme, NyxVariant } from 'nyx-kit/types'

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
  if (!confirm('This will scan all home directories and fix author_id mismatches in vaults and notes. Continue?')) return
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
  if (!confirm('Delete ALL notes across all vaults? This cannot be undone.')) return
  if (!confirm('Are you absolutely sure? This will permanently delete every note.')) return
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
  if (!confirm('Delete ALL vaults and their notes? This cannot be undone.')) return
  if (!confirm('Are you absolutely sure? This will permanently delete every vault and note.')) return
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
  if (!confirm('Delete ALL home directories (users\' personal vaults and notes)? This cannot be undone.')) return
  if (!confirm('Are you absolutely sure? This will permanently delete every home directory.')) return
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
  if (!confirm('Delete ALL server vaults and their notes? This cannot be undone.')) return
  if (!confirm('Are you absolutely sure? This will permanently delete every server vault.')) return
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
  <div class="servers-page">
    <div class="servers-page__inner">
      <header class="servers-page__header">
        <h1 class="servers-page__title">Settings</h1>
      </header>

      <div class="servers-page__content">
        <section class="servers-section">
          <div class="servers-section__header">
            <h2>Server</h2>
          </div>

          <div class="servers-section__settings">
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

        <section v-if="auth.isAuthenticated.value" class="servers-section">
          <div class="servers-section__header">
            <h2>Session</h2>
          </div>
          <div class="servers-section__actions">
            <NyxButton @click="signOut">Sign Out</NyxButton>
          </div>
        </section>

        <section class="servers-section">
          <div class="servers-section__header">
            <h2>Sync</h2>
          </div>
          <p class="servers-section__desc">
            Scan all home directories and fix author_id mismatches between the filesystem and note frontmatter.
          </p>
          <div class="servers-section__actions">
            <NyxButton :loading="syncing" @click="syncHomes">Sync All Homes</NyxButton>
          </div>
          <div v-if="syncResult" class="sync-result">
            <span>{{ syncResult.homes_scanned }} homes scanned</span>
            <span>{{ syncResult.vaults_fixed }} vaults fixed</span>
            <span>{{ syncResult.notes_fixed }} notes fixed</span>
          </div>
        </section>

        <section class="servers-section servers-section--danger">
          <div class="servers-section__header">
            <h2 class="danger-title">Danger Zone</h2>
          </div>
          <p class="servers-section__desc">
            These actions are destructive and cannot be undone.
          </p>

          <div class="danger-grid">
            <div class="danger-item">
              <div class="danger-item__info">
                <strong>Remove all notes</strong>
                <span>Delete every note across all vaults. Vaults remain.</span>
              </div>
              <NyxButton :loading="removingNotes" :theme="NyxTheme.Danger" :variant="NyxVariant.Soft" @click="removeAllNotes">Remove Notes</NyxButton>
            </div>

            <div class="danger-item">
              <div class="danger-item__info">
                <strong>Remove all vaults</strong>
                <span>Delete every vault and all notes inside them.</span>
              </div>
              <NyxButton :loading="removingVaults" :theme="NyxTheme.Danger" :variant="NyxVariant.Soft" @click="removeAllVaults">Remove Vaults</NyxButton>
            </div>

            <div class="danger-item">
              <div class="danger-item__info">
                <strong>Remove all homes</strong>
                <span>Delete all user home directories (personal vaults and notes).</span>
              </div>
              <NyxButton :loading="removingHomes" :theme="NyxTheme.Danger" :variant="NyxVariant.Soft" @click="removeAllHomes">Remove Homes</NyxButton>
            </div>

            <div class="danger-item">
              <div class="danger-item__info">
                <strong>Remove all server vaults</strong>
                <span>Delete all shared server vaults and their notes.</span>
              </div>
              <NyxButton :loading="removingServerVaults" :theme="NyxTheme.Danger" :variant="NyxVariant.Soft" @click="removeAllServerVaults">Remove Server Vaults</NyxButton>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<style scoped>
.servers-page {
  min-height: 100vh;
  padding: 2rem 1rem;
  overflow-y: auto;
}

.servers-page__header {
  margin-bottom: 2rem;
}

.servers-page__title {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
}

.servers-page__content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.servers-section {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-lg);
  padding: 1.25rem;
}

.servers-section--danger {
  border: 1px solid rgba(var(--nyx-rgb-danger), 0.3);
}

.servers-section__settings {
  display: grid;
  gap: 1rem;
}

.servers-section__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.servers-section__header h2 {
  font-size: 0.875rem;
  font-weight: 600;
  margin: 0;
}

.servers-section__actions {
  display: flex;
  gap: 0.5rem;
}

.servers-section__desc {
  font-size: 0.75rem;
  color: var(--nyx-c-text-3);
  margin: 0 0 1rem;
  line-height: 1.5;
}

.danger-title {
  color: var(--nyx-c-danger);
}

.danger-grid {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.danger-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.75rem;
  /* background: rgba(237, 137, 54, 0.05); */
  background: var(--nyx-c-bg-mute);
  border-radius: var(--nyx-radius-md);
  border: 1px solid rgba(237, 137, 54, 0.15);
}

.danger-item__info {
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
  min-width: 0;
  flex: 1;
}

.danger-item__info strong {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--nyx-c-text);
}

.danger-item__info span {
  font-size: 0.6875rem;
  color: var(--nyx-c-text-3);
}

.sync-result {
  display: flex;
  gap: 1rem;
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: var(--nyx-c-text-3);
}

</style>
