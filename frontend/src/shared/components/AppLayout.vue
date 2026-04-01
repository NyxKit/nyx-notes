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

    <!-- Main content area — renders the active child route -->
    <div class="app-shell__main">
      <!-- Global breadcrumbs -->
      <header v-if="breadcrumbs.length > 1" class="app-shell__breadcrumbs">
        <NyxBreadcrumbs :items="breadcrumbs" @click="onBreadcrumbClick" />
      </header>
      <RouterView v-slot="{ Component }">
        <component :is="Component">
          <template #header-actions>
            <slot name="header-actions" />
          </template>
        </component>
      </RouterView>
    </div>

  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: row;
  height: 100vh;
  overflow: hidden;
  background: var(--nyx-c-bg);
  color: var(--nyx-c-text-1);
}

/* ── Main column ─────────────────────────────────────────────── */
.app-shell__main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-width: 0;
}

/* ── Breadcrumbs ─────────────────────────────────────────────── */
.app-shell__breadcrumbs {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-c-divider);
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.app-shell__breadcrumbs :slotted(.header-actions) {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* ── Left sidebar ───────────────────────────────────────────── */
.app-shell__sidebar {
  width: 0;
  overflow: hidden;
  flex-shrink: 0;
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
</style>
