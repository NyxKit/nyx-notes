<script setup lang="ts">
import { onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/stores/vaults'
import { useNotesStore } from '@/stores/notes'
import VaultSwitcher from '@/components/VaultSwitcher.vue'
import SidebarNav from '@/components/SidebarNav.vue'
import NoteList from '@/components/NoteList.vue'

const vaultStore = useVaultStore()
const { vaults, activeVault } = storeToRefs(vaultStore)
const { load } = vaultStore
const { loadAll } = useNotesStore()

onMounted(async () => {
  await load()
  await loadAll(vaults.value.map(v => v.id))
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
        <div class="app-shell__sidebar-footer">
          <RouterLink
            v-if="activeVault"
            :to="`/vaults/${activeVault.id}/settings`"
            class="app-shell__footer-nav-item"
          >
            <svg width="15" height="15" viewBox="0 0 15 15" fill="none" aria-hidden="true">
              <circle cx="7.5" cy="7.5" r="2" stroke="currentColor" stroke-width="1.25"/>
              <path d="M7.5 1v1.5M7.5 12.5V14M1 7.5h1.5M12.5 7.5H14M2.75 2.75l1.06 1.06M11.19 11.19l1.06 1.06M2.75 12.25l1.06-1.06M11.19 3.81l1.06-1.06" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
            </svg>
            Settings
          </RouterLink>
          <a href="#" class="app-shell__footer-nav-item">
            <svg width="15" height="15" viewBox="0 0 15 15" fill="none" aria-hidden="true">
              <circle cx="7.5" cy="7.5" r="6" stroke="currentColor" stroke-width="1.25"/>
              <path d="M7.5 10.5v-1" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
              <path d="M7.5 8.5c0-1 .75-1.5 1.25-2A2.25 2.25 0 105.25 5" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
            </svg>
            Help
          </a>
        </div>
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

.app-shell__sidebar-footer {
  margin-top: auto;
  padding: 0.5rem 0.75rem;
  flex-shrink: 0;
  box-shadow: 0 -1px 0 0 var(--nyx-c-divider);
}

.app-shell__footer-nav-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  border-radius: var(--nyx-radius-md);
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
  text-decoration: none;
  transition: background 0.2s, color 0.2s;
}

.app-shell__footer-nav-item:hover {
  background: #25252b;
  color: var(--nyx-c-text-1);
}
</style>
