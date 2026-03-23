<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/stores/vaults'
import { useNotesStore } from '@/stores/notes'
import { NyxButton, NyxBadge } from 'nyx-kit/components'
import { NyxVariant, NyxTheme } from 'nyx-kit/types'

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

function openNote(noteId: string) {
  router.push(`/vaults/${vaultId.value}/notes/${noteId}`)
}

async function createFirst() {
  const meta = await createNote(vaultId.value, { title: '', content: '' })
  router.push(`/vaults/${vaultId.value}/notes/${meta.id}`)
}
</script>

<template>
  <div class="app-shell__main">

      <!-- Header -->
      <header class="app-shell__header">
        <div class="app-shell__header-left">
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

      <!-- Body -->
      <main class="app-shell__body">

        <!-- Loading -->
        <div v-if="listLoading" class="app-shell__canvas app-shell__canvas--center">
          <div class="vault__skeleton-grid">
            <div v-for="n in 6" :key="n" class="vault__skeleton-card" />
          </div>
        </div>

        <!-- Masonry -->
        <div v-else-if="sortedNotes.length > 0" class="app-shell__canvas app-shell__canvas--masonry">
          <div class="vault__masonry-header">
            <h2 class="vault__masonry-title">Notes</h2>
          </div>
          <div class="vault__masonry">
            <NyxButton
              v-for="note in sortedNotes"
              :key="note.id"
              class="vault__note-card"
              @click="openNote(note.id)"
            >
              <span class="vault__note-title">{{ note.title || 'Untitled' }}</span>
              <div v-if="note.tags.length" class="vault__note-tags">
                <NyxBadge v-for="tag in note.tags.slice(0, 3)" :key="tag" :theme="NyxTheme.Primary" :variant="NyxVariant.Soft">{{ tag }}</NyxBadge>
              </div>
              <span class="vault__note-date">{{ formatDate(note.updated_at) }}</span>
            </NyxButton>
          </div>
        </div>

        <!-- Empty state -->
        <div v-else class="app-shell__canvas app-shell__canvas--center">
          <div class="vault__welcome-card">
            <div class="vault__welcome-icon">
              <svg width="32" height="32" viewBox="0 0 32 32" fill="none" aria-hidden="true">
                <rect x="6" y="4" width="16" height="20" rx="2" stroke="currentColor" stroke-width="1.5"/>
                <path d="M10 10h8M10 14h8M10 18h5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              </svg>
            </div>
            <h1 class="vault__heading">This vault is empty.</h1>
            <p class="vault__desc">Start writing your first note. It will appear here once saved.</p>
            <NyxButton :gradient="true" @click="createFirst">New Note</NyxButton>
          </div>
        </div>

      </main>

      <!-- Footer -->
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

/* ── Header ─────────────────────────────────────────────────── */
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

/* ── Body ────────────────────────────────────────────────────── */
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

.app-shell__canvas--masonry {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  align-items: flex-start;
}

/* ── Footer ─────────────────────────────────────────────────── */
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

/* ── Masonry content ─────────────────────────────────────────── */
.vault__masonry-header {
  display: flex;
  align-items: center;
  width: 100%;
}

.vault__masonry-title {
  font-family: 'Manrope', sans-serif;
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--nyx-c-text-3);
  margin: 0;
}

.vault__masonry {
  columns: 3 220px;
  column-gap: 1rem;
  width: 100%;
}

.vault__note-card {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  padding: 1.25rem 1.25rem 1rem;
  margin-bottom: 1rem;
  break-inside: avoid;
  cursor: pointer;
  text-align: left;
  border: 1px solid transparent;
  width: 100%;
  transition: border-color 0.15s, background 0.15s;
}

.vault__note-card:hover {
  border-color: var(--nyx-c-divider);
  background: var(--nyx-c-bg-mute);
}

.vault__note-title {
  font-family: 'Manrope', sans-serif;
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--nyx-c-text-1);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.vault__note-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.vault__note-date {
  font-size: 0.6875rem;
  color: var(--nyx-c-text-3);
  margin-top: auto;
}

/* ── Skeleton ────────────────────────────────────────────────── */
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

/* ── Empty state ─────────────────────────────────────────────── */
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
