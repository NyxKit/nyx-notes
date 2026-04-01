<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useRoute, useRouter } from 'vue-router'
import { NyxButton, NyxGrid, NyxIcon } from 'nyx-kit/components'
import { NyxGridMode } from 'nyx-kit/types'
import { NoteCard } from '@/notes/components'
import { useNotesStore } from '@/notes/stores'
import { VaultIcon } from '@/vaults/components'
import { useVaultStore } from '@/vaults/stores'

const route = useRoute()
const router = useRouter()
const vaultId = computed(() => route.params.vault_id as string)

const vaultStore = useVaultStore()
const { vaults, activeVault } = storeToRefs(vaultStore)
const { load: loadVaults, setActive } = vaultStore
const notesStore = useNotesStore()
const { listLoading } = storeToRefs(notesStore)
const { notesFor, loadList, create: createNote } = notesStore

onMounted(async () => {
  await loadVaults()
  const vault = vaults.value.find(v => v.id === vaultId.value) ?? null
  if (vault) setActive(vault)
  await loadList(vaultId.value)
})

const sortedNotes = computed(() =>
  notesFor(vaultId.value).slice().sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
)

function formatDate(iso: string) {
  const d = new Date(iso)
  const now = Date.now()
  const diff = now - d.getTime()
  const mins = Math.floor(diff / 60_000)
  if (mins < 60) return mins <= 1 ? 'just now' : `${mins}m ago`
  const hrs = Math.floor(mins / 60)
  if (hrs < 24) return `${hrs}h ago`
  const days = Math.floor(hrs / 24)
  if (days < 7) return `${days}d ago`
  return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
}

async function createFirst() {
  const meta = await createNote(vaultId.value, { title: '', content: '' })
  router.push(`/vaults/${vaultId.value}/notes/${meta.id}`)
}
</script>

<template>
  <div class="app-shell__main">
    <header class="app-shell__header">
      <div class="app-shell__header-left">
        <VaultIcon :slug="activeVault?.icon" :size="18" />
        <span class="app-shell__title">{{ activeVault?.name ?? 'Vault' }}</span>
      </div>
      <div class="app-shell__header-right">
        <NyxButton
          v-if="sortedNotes.length > 0 && !listLoading"
          :gradient="true"
          @click="createFirst"
        >
          New Note
        </NyxButton>
      </div>
    </header>

    <main class="app-shell__body">
      <div v-if="listLoading" class="app-shell__canvas app-shell__canvas--center">
        <div class="vault__skeleton-grid">
          <div v-for="n in 6" :key="n" class="vault__skeleton-card" />
        </div>
      </div>

      <div v-else-if="sortedNotes.length > 0" class="app-shell__canvas app-shell__canvas--overview">
        <NyxGrid title="Notes" :mode="NyxGridMode.Masonry" :columns="5">
          <NoteCard
            v-for="note in sortedNotes"
            :key="note.id"
            :note="note"
            :vault-id="vaultId"
            :updated-label="formatDate(note.updated_at)"
          />
        </NyxGrid>
      </div>

      <div v-else class="app-shell__canvas app-shell__canvas--center">
        <div class="vault__welcome-card">
          <div class="vault__welcome-icon">
            <NyxIcon name="file-text" :size="32" />
          </div>
          <h1 class="vault__heading">This vault is empty.</h1>
          <p class="vault__desc">Start writing your first note. It will appear here once saved.</p>
          <NyxButton :gradient="true" @click="createFirst">New Note</NyxButton>
        </div>
      </div>
    </main>

    <footer class="app-shell__footer">
      <span class="app-shell__footer-text">{{ activeVault?.name ?? 'Vault' }}</span>
    </footer>
  </div>
</template>

<style scoped>
.app-shell__main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.app-shell__header {
  height: 64px;
  flex-shrink: 0;
  background: var(--nyx-c-bg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  box-shadow: 0 1px 0 0 var(--nyx-c-divider);
}

.app-shell__header-left,
.app-shell__header-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.app-shell__title {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
}

.app-shell__body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.app-shell__canvas {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.app-shell__canvas--center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-shell__canvas--overview {
  display: flex;
  flex-direction: column;
}

.app-shell__footer {
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.app-shell__footer-text {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

.vault__skeleton-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  max-width: 900px;
}

.vault__skeleton-card {
  height: 120px;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  animation: vault-pulse 1.4s ease-in-out infinite;
}

@keyframes vault-pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}

.vault__welcome-card {
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 2.5rem 2rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  align-items: flex-start;
  max-width: 400px;
  width: 100%;
}

.vault__welcome-icon {
  color: var(--nyx-c-primary);
  opacity: 0.7;
  line-height: 0;
}

.vault__heading {
  font-family: 'Manrope', sans-serif;
  font-size: 1.5rem;
  font-weight: 700;
  line-height: 1.25;
  color: var(--nyx-c-text-1);
  margin: 0;
}

.vault__desc {
  font-size: 0.875rem;
  line-height: 1.6;
  color: var(--nyx-c-text-2);
  margin: 0;
}
</style>
