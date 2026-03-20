<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'

const route = useRoute()
const router = useRouter()
const { activeVault } = useVaults()
const { create } = useNotes()

const section = computed(() => {
  if (route.path.includes('/favorites')) return 'favorites'
  if (route.path.includes('/drafts')) return 'drafts'
  return 'notes'
})

const vaultId = computed(() => activeVault.value?.id ?? '')

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
      <button class="sidebar-nav__new-btn" @click="newNote">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
          <path d="M7 2v10M2 7h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        New Note
      </button>
    </div>

    <!-- Workspace section -->
    <div class="sidebar-nav__section-label">Workspace</div>

    <RouterLink
      v-if="vaultId"
      :to="`/vaults/${vaultId}/notes`"
      class="sidebar-nav__item"
      :class="{ 'sidebar-nav__item--active': section === 'notes' }"
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

.sidebar-nav__new-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.625rem 1rem;
  border-radius: var(--nyx-radius-md);
  background: linear-gradient(135deg, #cbc2e4 0%, #49435f 100%);
  color: #1a1821;
  font-size: 0.8125rem;
  font-weight: 600;
  font-family: 'Manrope', sans-serif;
  border: none;
  cursor: pointer;
  transition: opacity 0.2s;
}

.sidebar-nav__new-btn:hover {
  opacity: 0.9;
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
