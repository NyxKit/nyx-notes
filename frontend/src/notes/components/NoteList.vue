<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useNoteBrowsingStore } from '@/notes/stores'
import { useAuth } from '@/auth/composables'
import { useWorkspaceProfiles } from '@/shared/composables'
import { NyxInput } from 'nyx-kit/components'
import { NyxInputType } from 'nyx-kit/types'

const RECENT_LIMIT = 20

const router = useRouter()
const route = useRoute()
const { apiEpoch } = useAuth()
const { activeProfile, profiles } = useWorkspaceProfiles()
const noteBrowsingStore = useNoteBrowsingStore()
const { recentResults } = storeToRefs(noteBrowsingStore)
const { loadRecentNotes } = noteBrowsingStore

const search = ref(String(route.query.q ?? ''))
let searchTimeout: ReturnType<typeof setTimeout> | null = null
let syncingFromRoute = false

const recentNotes = computed(() => {
  return recentResults.value.slice(0, RECENT_LIMIT)
})

watch([apiEpoch, profiles, activeProfile], async () => {
  await loadRecentNotes()
}, { immediate: true, deep: true })

watch(() => [route.path, String(route.query.q ?? '')] as const, ([path, value]) => {
  if (path !== '/notes/search') return
  syncingFromRoute = true
  search.value = value
})

watch(search, (value) => {
  if (syncingFromRoute) {
    syncingFromRoute = false
    return
  }

  if (searchTimeout) clearTimeout(searchTimeout)

  searchTimeout = setTimeout(() => {
    router.replace({
      path: '/notes/search',
      query: value ? { q: value } : {},
    })
  }, 200)
})

onBeforeUnmount(() => {
  if (searchTimeout) clearTimeout(searchTimeout)
})

</script>

<template>
  <div class="note-list">

    <!-- Search -->
    <div class="note-list__search-wrap">
      <NyxInput
        v-model="search"
        :type="NyxInputType.Search"
        placeholder="Search notes…"
      />
    </div>

    <!-- Section label -->
    <div class="note-list__section-label">Recent Notes</div>

    <div v-if="recentNotes.length === 0" class="note-list__empty">
      No notes yet
    </div>

    <ul v-else class="note-list__items">
      <li
        v-for="note in recentNotes"
        :key="`${note.profile_id}:${note.vault_id}:${note.note_id}`"
        class="note-list__item"
        :class="{ 'note-list__item--active': route.params.id === note.note_id && String(route.query.profile ?? activeProfile?.id ?? 'local') === note.profile_id }"
        @click="router.push(note.href)"
      >
        <div class="note-list__title">
          {{ note.title || 'Untitled' }}
        </div>
        <div class="note-list__meta">
          <span class="note-list__origin">{{ note.server_label }} / {{ note.vault_name }}</span>
          <span class="note-list__time">{{ note.updated_label }}</span>
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
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  font-size: 0.6875rem;
  color: var(--nyx-c-text-3);
  margin-top: 0.125rem;
}

.note-list__origin {
  min-width: 0;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.note-list__time {
  flex-shrink: 0;
}
</style>
