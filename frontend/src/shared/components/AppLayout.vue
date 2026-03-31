<script setup lang="ts">
import { watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { useTeams } from '@/teams/composables'
import { useComments } from '@/comments/composables'
import { VaultSwitcher } from '@/vaults/components'
import { SidebarNav } from '@/shared/components'
import { NoteList } from '@/notes/components'

const vaultStore = useVaultStore()
const notesStore = useNotesStore()
const teams = useTeams()
const comments = useComments()
const { apiEpoch, isAuthenticated } = useAuth()
const { activeProfile } = useWorkspaceProfiles()
const { vaults } = storeToRefs(vaultStore)
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
      <RouterView />
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
