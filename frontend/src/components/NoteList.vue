<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { NyxButton, NyxInput } from 'nyx-kit/components'
import { NyxInputType } from 'nyx-kit/types'
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
    <div class="note-list__header">
      <NyxInput
        v-model="search"
        :type="NyxInputType.Search"
        placeholder="Search notes…"
      />
      <NyxButton @click="newNote">New</NyxButton>
    </div>

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
        <div v-if="note.tags.length" class="note-list__tags">
          <span v-for="tag in note.tags" :key="tag" class="note-list__tag">{{ tag }}</span>
        </div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.note-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.note-list__header {
  display: flex;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.note-list__header :deep(input) {
  flex: 1;
}

.note-list__empty {
  padding: 2rem 1rem;
  text-align: center;
  font-size: 0.875rem;
  color: var(--nyx-color-muted, #718096);
}

.note-list__items {
  list-style: none;
  margin: 0;
  padding: 0;
  overflow-y: auto;
  flex: 1;
}

.note-list__item {
  padding: 0.75rem 1rem;
  cursor: pointer;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
  transition: background 0.1s;
}

.note-list__item:hover,
.note-list__item--active {
  background: var(--nyx-color-surface-hover, #f7fafc);
}

.note-list__title {
  font-size: 0.875rem;
  font-weight: 500;
  display: flex;
  justify-content: space-between;
}

.note-list__meta {
  display: flex;
  gap: 0.5rem;
  font-size: 0.75rem;
  color: var(--nyx-color-muted, #718096);
  margin-top: 0.125rem;
}

.note-list__tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  margin-top: 0.25rem;
}

.note-list__tag {
  font-size: 0.625rem;
  padding: 0.125rem 0.375rem;
  border-radius: 9999px;
  background: var(--nyx-color-surface-muted, #edf2f7);
  color: var(--nyx-color-muted, #718096);
}
</style>
