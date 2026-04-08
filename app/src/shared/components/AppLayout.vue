<script setup lang="ts">
import { watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxButton } from 'nyx-kit/components'
import { NyxTheme } from 'nyx-kit/types'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { RouteName } from '@/shared/types'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { VaultSwitcher } from '@/vaults/components'
import AppBreadcrumbs from './AppBreadcrumbs.vue'
import SidebarNav from './SidebarNav.vue'
import SidebarNavItem from './SidebarNavItem.vue'
import { NoteList, NoteSearch } from '@/notes/components'
import type { Vault } from '@/shared/types'
import { useFeedbackDialog } from '@/feedback/composables'
import { FeedbackModal } from '@/feedback/views'

const route = useRoute()
const router = useRouter()
const vaultStore = useVaultStore()
const notesStore = useNotesStore()
const { apiEpoch, isAuthenticated, authMode, serverMetadata } = useAuth()
const { activeProfile } = useWorkspaceProfiles()
const { openFeedbackDialog } = useFeedbackDialog()
const { vaults } = storeToRefs(vaultStore)
const { load } = vaultStore
const { loadAll } = notesStore

async function refreshWorkspace() {
  if (!isAuthenticated.value) return

  await load()
  await loadAll(vaults.value.map(v => v.slug))

  if (route.params.vault_id) {
    const currentVault = vaults.value.find(v => v.slug === route.params.vault_id) ?? null
    if (currentVault) {
      vaultStore.setActive(currentVault as Vault)
    }
  }
}

function openFeedback() {
  if (serverMetadata.value?.role === 'admin') {
    router.push({ name: RouteName.Feedback, params: { server_slug: serverMetadata.value.slug } })
    return
  }

  openFeedbackDialog()
}

watch([activeProfile, apiEpoch], async () => {
  await refreshWorkspace()
}, {
  immediate: true,
})

watch(
  [() => route.params.vault_id as string | undefined, vaults],
  ([vaultId]) => {
    if (!vaultId) {
      return
    }

    const currentVault = vaults.value.find(v => v.slug === vaultId) ?? null
    if (currentVault) {
      vaultStore.setActive(currentVault as Vault)
    }
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
        <SidebarNavItem
          v-if="authMode === 'secret_key' && serverMetadata?.role === 'admin'"
          :to="{ name: RouteName.Users }"
          icon="users"
          class="app-shell__settings-link app-shell__users-link"
          :active="route.name === RouteName.Users"
        >
          Users
        </SidebarNavItem>
        <SidebarNavItem
          :to="{ name: RouteName.Settings }"
          icon="settings"
          class="app-shell__settings-link"
          :active="route.name === RouteName.Settings"
        >
          Settings
        </SidebarNavItem>

        <NyxButton
          class="app-shell__feedback-button"
          :theme="NyxTheme.Primary"
          :gradient="true"
          type="button"
          @click="openFeedback"
        >
          Feedback
        </NyxButton>
      </div>
    </aside>

    <!-- Global header -->
    <header id="header" class="app-shell__header">
      <AppBreadcrumbs />
      <div id="layout-header-actions" class="app-shell__header-actions" />
    </header>

    <RouterView class="app-shell__body" />
    <FeedbackModal />
  </div>
</template>

<style scoped>
.app-shell__settings-link {
  margin: 0 0.75rem 0.75rem;
  border-top: 1px solid var(--nyx-c-divider);
  flex-shrink: 0;
}

.app-shell__feedback-button {
  margin: 0 0.75rem 0.75rem;
  flex-shrink: 0;
}

.app-shell__users-link {
  margin-bottom: 0.5rem;
}

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
  width: max(288px, 15dvw);
  min-width: 0;
  max-height: 100dvh;
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.app-shell__sidebar-inner {
  width: 100%;
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
