<script setup lang="ts">
import { computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxBreadcrumbs } from 'nyx-kit/components'
import type { NyxBreadcrumb } from 'nyx-kit/types'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { useTeams } from '@/teams/composables'
import { useComments } from '@/comments/composables'
import { VaultSwitcher } from '@/vaults/components'
import { SidebarNav } from '@/shared/components'
import { NoteList } from '@/notes/components'

const route = useRoute()
const router = useRouter()
const vaultStore = useVaultStore()
const notesStore = useNotesStore()
const teams = useTeams()
const comments = useComments()
const { apiEpoch, isAuthenticated } = useAuth()
const { activeProfile } = useWorkspaceProfiles()
const { vaults } = storeToRefs(vaultStore)
const { activeNote } = storeToRefs(notesStore)
const { load } = vaultStore
const { loadAll } = notesStore

async function refreshWorkspace() {
  vaultStore.$reset()
  notesStore.$reset()
  teams.clear()
  comments.clearLoadedComments()

  if (!isAuthenticated.value) return

  await load()
  await loadAll(vaults.value.map(v => v.id))
}

watch([activeProfile, apiEpoch], async () => {
  await refreshWorkspace()
}, {
  immediate: true,
})

const breadcrumbs = computed((): NyxBreadcrumb[] => {
  const path = route.path

  if (path === '/' || path === '/vaults') {
    return []
  }

  const items: NyxBreadcrumb[] = [{ label: 'All notes', href: '/notes/search' }]

  if (path.includes('/favorites')) {
    items.push({ label: 'Favorites', href: '/notes/favorites' })
  }

  if (path.includes('/vaults/') && route.params.vault_id) {
    const vaultId = route.params.vault_id as string
    const vault = vaults.value.find(v => v.id === vaultId)
    if (vault) {
      items.push({ label: vault.name, href: `/vaults/${vault.id}` })
    }
  }

  if (path.includes('/notes/') && route.params.id && activeNote.value) {
    items.push({ label: activeNote.value.meta.title || 'Untitled' })
  }

  return items
})

function onBreadcrumbClick(item: NyxBreadcrumb) {
  if (item.href) router.push(item.href)
}
</script>

<template>
  <div class="app-shell">

    <!-- Left sidebar — persistent across all authenticated views -->
    <aside class="app-shell__sidebar app-shell__sidebar--open">
      <div class="app-shell__sidebar-inner">
        <VaultSwitcher />
        <SidebarNav />
        <NoteList />
      </div>
    </aside>

    <!-- Global header -->
    <header class="app-shell__header">
      <NyxBreadcrumbs :items="breadcrumbs" @click="onBreadcrumbClick" />
      <!-- Nested routes cannot fill parent slots; views use Teleport here. -->
      <div id="app-shell-header-actions" class="app-shell__header-actions" />
    </header>

    <RouterView class="app-shell__body" />
  </div>
</template>

<style scoped>
.app-shell {
  --app-shell-header-height: 3.25rem;

  display: grid;
  grid-template-columns: auto 1fr;
  grid-template-rows: var(--app-shell-header-height) 1fr;
  height: 100vh;
  overflow: hidden;
  background: var(--nyx-c-bg);
  color: var(--nyx-c-text-1);
}

/* ── Left sidebar — spans header + main rows ─────────────────── */
.app-shell__sidebar {
  grid-column: 1;
  grid-row: 1 / -1;
  width: 0;
  overflow: hidden;
  min-width: 0;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.app-shell__sidebar--open {
  width: 288px;
}

.app-shell__sidebar-inner {
  width: 288px;
  height: 100%;
  background: var(--nyx-c-bg-soft);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ── Global header — fixed row height ────────────────────────── */
.app-shell__header {
  grid-column: 2;
  grid-row: 1;
  box-sizing: border-box;
  height: 100%;
  min-width: 0;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-c-divider);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.app-shell__header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 0;
}

/* ── Main — fills remaining viewport; scrolls internally ─────── */
.app-shell__body {
  grid-column: 2;
  grid-row: 2;
  min-height: 0;
  min-width: 0;
  overflow: auto;
  display: flex;
  flex-direction: column;
}
</style>
