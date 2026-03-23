<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useVaultStore } from '@/stores/vaults'
import { useNotesStore } from '@/stores/notes'
import { NyxButton } from 'nyx-kit/components'

const route = useRoute()
const router = useRouter()
const { activeVault } = storeToRefs(useVaultStore())
const { create } = useNotesStore()

const vaultId = computed(() => activeVault.value?.id ?? '')

const section = computed(() => {
  if (route.path === '/') return 'home'
  if (route.path.includes('/favorites')) return 'favorites'
  if (route.path.includes('/drafts')) return 'drafts'
  if (vaultId.value && route.path === `/vaults/${vaultId.value}`) return 'vault'
  return 'notes'
})

async function newNote() {
  if (!vaultId.value) return
  const meta = await create(vaultId.value, { title: '', content: '' })
  router.push(`/vaults/${vaultId.value}/notes/${meta.id}`)
}
</script>

<template>
  <nav class="sidebar-nav">

    <!-- New Note CTA -->
    <div class="sidebar-nav__cta">
      <NyxButton :gradient="true" style="width: 100%" @click="newNote">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
          <path d="M7 2v10M2 7h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        New Note
      </NyxButton>
    </div>

    <!-- App section -->
    <div class="sidebar-nav__section-label">App</div>

    <RouterLink
      to="/"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'home' }"
    >
      <!-- Vault / grid icon -->
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <rect x="2" y="2" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.25"/>
        <rect x="9" y="2" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.25"/>
        <rect x="2" y="9" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.25"/>
        <rect x="9" y="9" width="5" height="5" rx="1" stroke="currentColor" stroke-width="1.25"/>
      </svg>
      Vaults
    </RouterLink>

    <!-- Workspace section -->
    <div v-if="vaultId" class="sidebar-nav__section-label">Workspace</div>

    <RouterLink
      v-if="vaultId"
      :to="`/vaults/${vaultId}`"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'vault' }"
    >
      <!-- Document icon -->
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <rect x="3" y="2" width="10" height="12" rx="1.5" stroke="currentColor" stroke-width="1.25"/>
        <path d="M5.5 6h5M5.5 8.5h5M5.5 11h3" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
      </svg>
      All Notes
    </RouterLink>

    <RouterLink
      v-if="vaultId"
      :to="`/vaults/${vaultId}/favorites`"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'favorites' }"
    >
      <!-- Star icon -->
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <path d="M8 2l1.5 4H14l-3.5 2.5 1.5 4.5L8 10.5 4 13l1.5-4.5L2 6h4.5L8 2z" stroke="currentColor" stroke-width="1.25" stroke-linejoin="round"/>
      </svg>
      Favorites
    </RouterLink>

    <RouterLink
      v-if="vaultId"
      :to="`/vaults/${vaultId}/drafts`"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'drafts' }"
    >
      <!-- Edit icon -->
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <path d="M10 3l3 3-7 7H3v-3l7-7z" stroke="currentColor" stroke-width="1.25" stroke-linejoin="round"/>
        <path d="M8.5 4.5l3 3" stroke="currentColor" stroke-width="1.25" stroke-linecap="round"/>
      </svg>
      Drafts
    </RouterLink>

  </nav>
</template>

<style scoped>
.sidebar-nav {
  padding: 0 0.75rem;
  flex-shrink: 0;
}

/* New Note CTA */
.sidebar-nav__cta {
  padding: 0.5rem 0.25rem 0.75rem;
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
