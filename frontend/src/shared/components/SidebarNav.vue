<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/vaults/stores'
import { NyxIcon } from 'nyx-kit/components'

const route = useRoute()
const { activeVault } = storeToRefs(useVaultStore())

const vaultId = computed(() => activeVault.value?.id ?? '')

const section = computed(() => {
  if (route.path === '/') return 'home'
  if (route.path === '/notes/favorites') return 'favorites'
  if (route.path.includes('/drafts')) return 'drafts'
  if (vaultId.value && route.path === `/vaults/${vaultId.value}`) return 'vault'
  return 'notes'
})
</script>

<template>
  <nav class="sidebar-nav">

    <!-- App section -->
    <div class="sidebar-nav__section-label">App</div>

    <RouterLink
      to="/"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'home' }"
    >
      <NyxIcon name="layout-grid" :size="16" />
      Vaults
    </RouterLink>

    <RouterLink
      to="/servers"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': route.path === '/servers' }"
    >
      <NyxIcon name="server" :size="16" />
      Servers
    </RouterLink>

    <!-- Workspace section -->
    <div class="sidebar-nav__section-label">Workspace</div>

    <RouterLink
      v-if="vaultId"
      :to="`/vaults/${vaultId}`"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'vault' }"
    >
      <NyxIcon name="file-text" :size="16" />
      All Notes
    </RouterLink>

    <RouterLink
      to="/notes/favorites"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'favorites' }"
    >
      <NyxIcon name="star" :size="16" />
      Favorites
    </RouterLink>

    <RouterLink
      v-if="vaultId"
      :to="`/vaults/${vaultId}/drafts`"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'drafts' }"
    >
      <NyxIcon name="edit-3" :size="16" />
      Drafts
    </RouterLink>

  </nav>
</template>

<style scoped>
.sidebar-nav {
  padding: 0 0.75rem;
  flex-shrink: 0;
}

/* Section label */
.sidebar-nav__section-label {
  padding: 0.5rem 0.5rem 0.25rem;
  font-size: 0.625rem;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--nyx-c-text-3);
}

/* Nav items */
.sidebar-nav__item {
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

.sidebar-nav__item:hover {
  background: #25252b;
  color: var(--nyx-c-text-1);
}

.sidebar-nav__item--active {
  background: #49435f;
  color: var(--nyx-c-text-1);
}
</style>
