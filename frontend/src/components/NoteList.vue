<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useVaults } from '@/composables/useVaults'
import { useNotes } from '@/composables/useNotes'
import { useAuth } from '@/composables/useAuth'
import { getRelativeTime } from '@/utils/time'
import { NyxInput } from 'nyx-kit/components'

const router = useRouter()
const { activeVault } = useVaults()
const { notes, listLoading, loadList, activeNote } = useNotes()
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

function permissionIcon(permission: string): string {
  if (permission === 'comment') return '💬'
  if (permission === 'edit') return '✏️'
  return ''
}
</script>

<template>
  <div class="note-list">

    <!-- Search -->
    <div class="note-list__search-wrap">
      <NyxInput
        v-model="search"
        type="search"
        placeholder="Search notes…"
      />
    </div>

    <!-- Section label -->
    <div class="note-list__section-label">Recent Notes</div>

    <div v-if="listLoading" class="note-list__empty">Loading…</div>

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

/* Search */
.note-list__search-wrap {
  padding: 0.25rem 1rem 0.5rem;
  flex-shrink: 0;
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
