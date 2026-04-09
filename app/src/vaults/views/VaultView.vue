<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useRoute, useRouter } from 'vue-router'
import { NyxButton, NyxGrid, NyxIcon } from 'nyx-kit/components'
import { NyxGridMode } from 'nyx-kit/types'
import { noteRoute } from '@/shared/utils'
import NoteCard from '@/notes/components/NoteCard.vue'
import { useNoteBrowsingStore, useNotesStore } from '@/notes/stores'
import { useSubscription, useWorkspaceProfiles } from '@/shared/composables'
import type { BrowseNoteCardModel } from '@/shared/types'
import { WorkspaceProfileType } from '@/shared/types'
import { useVaults } from '@/vaults/composables'
import { useVaultStore } from '@/vaults/stores'

const route = useRoute()
const router = useRouter()
const vaultId = computed(() => route.params.vault_id as string)
const { activeProfile } = useWorkspaceProfiles()
const serverSlug = computed(() => activeProfile.value?.id || 'main-server')
useVaults()

const vaultStore = useVaultStore()
const { vaults, activeVault } = storeToRefs(vaultStore)
const { load: loadVaults, setActive } = vaultStore
const notesStore = useNotesStore()
const noteBrowsingStore = useNoteBrowsingStore()
const { listLoading } = storeToRefs(notesStore)
const { notesFor, loadList, subscribeList, create: createNote } = notesStore

useSubscription(vaultId, value => subscribeList(serverSlug.value, value))

onMounted(async () => {
  await loadVaults()
  const vault = vaults.value.find(v => v.slug === vaultId.value) ?? null
  if (vault) setActive(vault)
  await loadList(vaultId.value)
})

const sortedNotes = computed<BrowseNoteCardModel[]>(() =>
  notesFor(vaultId.value)
    .slice()
    .sort((a, b) => {
      const favorite = Number(noteBrowsingStore.isFavorite(activeProfile.value?.id ?? 'local', b.vault_id, b.id))
        - Number(noteBrowsingStore.isFavorite(activeProfile.value?.id ?? 'local', a.vault_id, a.id))
      if (favorite !== 0) return favorite

      return new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
    })
    .map(note => ({
        note_id: note.id,
        vault_id: note.vault_id,
        profile_id: activeProfile.value?.id ?? 'local',
        title: note.title || 'Untitled',
        description: note.description,
        tags: note.tags,
        images: note.images ?? [],
        updated_at: note.updated_at,
        updated_label: formatDate(note.updated_at),
        href: noteRoute(activeVault.value!, note.id),
        server_label: activeProfile.value?.display_name ?? 'Main Server',
        server_id: activeProfile.value?.type === WorkspaceProfileType.Remote ? activeProfile.value.server_id : undefined,
        vault_name: activeVault.value?.name ?? 'Vault',
        vault_slug: activeVault.value?.slug ?? '',
        is_favorite: noteBrowsingStore.isFavorite(activeProfile.value?.id ?? 'local', note.vault_id, note.id),
    }))
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
  if (activeVault.value) {
    router.push(noteRoute(activeVault.value, meta.id))
  }
}
</script>

<template>
  <div class="vault-view">
    <Teleport to="#layout-header-actions" defer>
      <NyxButton
        v-if="sortedNotes.length > 0 && !listLoading"
        :gradient="true"
        @click="createFirst"
      >
        New Note
      </NyxButton>
    </Teleport>

    <main class="vault-view__body">
      <div v-if="listLoading" class="vault-view__canvas vault-view__canvas--center">
        <div class="vault-view__skeleton-grid">
          <div v-for="n in 6" :key="n" class="vault-view__skeleton-card" />
        </div>
      </div>

      <div v-else-if="sortedNotes.length > 0" class="vault-view__canvas vault-view__canvas--overview">
        <NyxGrid title="Notes" :mode="NyxGridMode.Masonry" :columns="5">
          <NoteCard
            v-for="note in sortedNotes"
            :key="note.note_id"
            :note="note"
            :image="note.images?.[0]"
          />
        </NyxGrid>
      </div>

      <div v-else class="vault-view__canvas vault-view__canvas--center">
        <div class="vault-view__welcome-card">
          <div class="vault-view__welcome-icon">
            <NyxIcon name="file-text" :size="32" />
          </div>
          <h1 class="vault-view__heading">This vault is empty.</h1>
          <p class="vault-view__desc">Start writing your first note. It will appear here once saved.</p>
          <NyxButton :gradient="true" @click="createFirst">New Note</NyxButton>
        </div>
      </div>
    </main>

    <footer class="vault-view__footer">
      <span class="vault-view__footer-text">{{ activeVault?.name ?? 'Vault' }}</span>
    </footer>
  </div>
</template>

<style scoped>
.vault-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

.vault-view__body {
  flex: 1;
  overflow: hidden;
  display: flex;
}

.vault-view__canvas {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.vault-view__canvas--center {
  display: flex;
  align-items: center;
  justify-content: center;
}

.vault-view__canvas--overview {
  display: flex;
  flex-direction: column;
}

.vault-view__footer {
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1.5rem;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.vault-view__footer-text {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

.vault-view__skeleton-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  width: 100%;
  max-width: 900px;
}

.vault-view__skeleton-card {
  height: 120px;
  background: var(--nyx-c-bg-soft);
  border-radius: var(--nyx-radius-xl);
  animation: vault-view-pulse 1.4s ease-in-out infinite;
}

@keyframes vault-view-pulse {
  0%, 100% { opacity: 1 }
  50% { opacity: 0.4 }
}

.vault-view__welcome-card {
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

.vault-view__welcome-icon {
  color: var(--nyx-c-primary);
  opacity: 0.7;
  line-height: 0;
}

.vault-view__heading {
  font-family: 'Manrope', sans-serif;
  font-size: 1.5rem;
  font-weight: 700;
  line-height: 1.25;
  color: var(--nyx-c-text-1);
  margin: 0;
}

.vault-view__desc {
  font-size: 0.875rem;
  line-height: 1.6;
  color: var(--nyx-c-text-2);
  margin: 0;
}
</style>
