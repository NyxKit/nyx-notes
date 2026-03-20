<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'
import { useAuth } from '@/composables/useAuth'
import { getRelativeTime } from '@/utils/time'

const router = useRouter()
const { activeVault } = useVaults()
const { notes, loading, loadList, create, activeNote } = useNotes()
const { currentUser } = useAuth()

const search = ref('')

const filtered = computed(() => {
  const q = search.value.toLowerCase()
  if (!q) return notes.value
  return notes.value.filter(n =>
    n.title.toLowerCase().includes(q) ||
    n.tags.some(t => t.toLowerCase().includes(q))
  )
})

watch(activeVault, vault => {
  if (vault) loadList(vault.id)
}, { immediate: true })

async function newNote() {
  if (!activeVault.value) return
  const meta = await create(activeVault.value.id, {
    title: 'Untitled',
    content: '',
  })
  router.push(`/vaults/${activeVault.value.id}/notes/${meta.id}`)
}

function permissionIcon(permission: string): string {
  if (permission === 'comment') return '💬'
  if (permission === 'edit') return '✏️'
  return ''
}
</script>

<template>
  <div class="note-list">

    <!-- New note CTA -->
    <div class="note-list__cta">
      <button class="note-list__new-btn" @click="newNote">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
          <path d="M7 2v10M2 7h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        New Note
      </button>
    </div>

    <!-- Search -->
    <div class="note-list__search-wrap">
      <input
        v-model="search"
        class="note-list__search"
        type="search"
        placeholder="Search notes…"
      />
    </div>

    <!-- Section label -->
    <div class="note-list__section-label">Notes</div>

    <div v-if="loading" class="note-list__empty">Loading…</div>

    <div v-else-if="filtered.length === 0" class="note-list__empty">
      {{ search ? 'No results' : 'No notes yet' }}
    </div>

    <ul v-else class="note-list__items">
      <li
        v-for="note in filtered"
        :key="note.id"
        class="note-list__item"
        :class="{ 'note-list__item--active': activeNote?.meta.id === note.id }"
        @click="router.push(`/vaults/${note.vault_id}/notes/${note.id}`)"
      >
        <div class="note-list__title">
          {{ note.title || 'Untitled' }}
          <span v-if="note.author_id !== currentUser?.id" class="note-list__permission">
            {{ permissionIcon(note.permission) }}
          </span>
        </div>
        <div class="note-list__meta">
          <span class="note-list__time">{{ getRelativeTime(note.updated_at) }}</span>
          <span v-if="note.category" class="note-list__category">{{ note.category }}</span>
        </div>
      </li>
    </ul>

  </div>
</template>

<style scoped>
.note-list {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

/* New note CTA */
.note-list__cta {
  padding: 0.75rem 1rem 0.5rem;
  flex-shrink: 0;
}

.note-list__new-btn {
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

.note-list__new-btn:hover {
  opacity: 0.9;
}

/* Search */
.note-list__search-wrap {
  padding: 0.25rem 1rem 0.5rem;
  flex-shrink: 0;
}

.note-list__search {
  width: 100%;
  background: rgba(37, 37, 43, 0.4);
  border: 1px solid var(--nyx-c-divider);
  border-radius: var(--nyx-radius-md);
  padding: 0.4375rem 0.75rem;
  font-size: 0.8125rem;
  color: var(--nyx-c-text-1);
  font-family: inherit;
  outline: none;
  transition: border-color 0.2s;
}

.note-list__search::placeholder {
  color: var(--nyx-c-text-3);
}

.note-list__search:focus {
  border-color: rgba(71, 71, 77, 0.5);
}

/* Section label */
.note-list__section-label {
  padding: 0.5rem 1rem 0.25rem;
  font-size: 0.625rem;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--nyx-c-text-3);
  flex-shrink: 0;
}

/* States */
.note-list__empty {
  padding: 2rem 1rem;
  text-align: center;
  font-size: 0.875rem;
  color: var(--nyx-c-text-3);
}

/* List */
.note-list__items {
  list-style: none;
  margin: 0;
  padding: 0;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}

.note-list__item {
  padding: 0.625rem 1rem 0.625rem calc(1rem - 2px);
  cursor: pointer;
  border-left: 2px solid transparent;
  transition: background 0.2s, border-color 0.2s;
}

.note-list__item:hover {
  background: var(--nyx-c-bg-mute);
}

.note-list__item--active {
  background: rgba(73, 67, 95, 0.4);
  border-left-color: var(--nyx-c-primary);
}

.note-list__title {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--nyx-c-text-1);
  display: flex;
  justify-content: space-between;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-list__item:not(.note-list__item--active) .note-list__title {
  color: var(--nyx-c-text-2);
}

.note-list__meta {
  display: flex;
  gap: 0.5rem;
  font-size: 0.6875rem;
  color: var(--nyx-c-text-3);
  margin-top: 0.125rem;
}

.note-list__permission {
  flex-shrink: 0;
}
</style>
