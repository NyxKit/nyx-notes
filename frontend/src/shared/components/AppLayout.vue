<script setup lang="ts">
import { watch } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { VaultSwitcher } from '@/vaults/components'
import AppBreadcrumbs from './AppBreadcrumbs.vue'
import SidebarNav from './SidebarNav.vue'
import { NoteList, NoteSearch } from '@/notes/components'
import type { Vault } from '@/shared/types'

const route = useRoute()
const vaultStore = useVaultStore()
const notesStore = useNotesStore()
const { apiEpoch, isAuthenticated } = useAuth()
const { activeProfile } = useWorkspaceProfiles()
const { vaults } = storeToRefs(vaultStore)
const { load } = vaultStore
const { loadAll } = notesStore

async function refreshWorkspace() {
  if (!isAuthenticated.value) return

  await load()
  await loadAll(vaults.value.map(v => v.id))

  if (route.params.vault_id) {
    const currentVault = vaults.value.find(v => v.id === route.params.vault_id) ?? null
    vaultStore.setActive(currentVault as Vault | null)
  }
}

watch([activeProfile, apiEpoch], async () => {
  await refreshWorkspace()
}, {
  immediate: true,
})

watch(
  [() => route.params.vault_id as string | undefined, vaults],
  ([vaultId]) => {
    if (!vaultId) return
    const currentVault = vaults.value.find(v => v.id === vaultId) ?? null
    vaultStore.setActive(currentVault as Vault | null)
  },
  { immediate: true }
)
</script>

<template>
  <div class="app-shell">

    <!-- Left sidebar — persistent across all authenticated views -->
    <aside id="sidebar" class="app-shell__sidebar app-shell__sidebar--open">
      <div class="app-shell__sidebar-inner">
        <VaultSwitcher />
        <NoteSearch />
        <SidebarNav />
        <NoteList />
      </div>
    </aside>

    <!-- Global header -->
    <header id="header" class="app-shell__header">
      <AppBreadcrumbs />
      <div id="layout-header-actions" class="app-shell__header-actions" />
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
