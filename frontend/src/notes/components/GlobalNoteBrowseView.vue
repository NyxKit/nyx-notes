<script setup lang="ts">
import { computed } from 'vue'
import { NyxGrid, NyxIcon, NyxSelect } from 'nyx-kit/components'
import { NyxGridMode } from 'nyx-kit/types'
import type { NyxSelectOption } from 'nyx-kit/types'
import NoteCard from './NoteCard.vue'
import type { BrowseNoteCardModel } from '@/shared/types'

const props = defineProps<{
  title?: string
  notes: BrowseNoteCardModel[]
  loading: boolean
  emptyTitle: string
  emptyMessage: string
  excludedProfilesCount: number
  sortModelValue: string
  sortOptions: NyxSelectOption[]
  error?: string | null
}>()

const emit = defineEmits<{
  'update:sortModelValue': [value: string]
}>()

const sortValue = computed({
  get: () => props.sortModelValue,
  set: (value: string) => emit('update:sortModelValue', value),
})
</script>

<template>
  <div class="browse-view">
    <Teleport to="#layout-header-actions" defer>
      <NyxSelect
        v-model="sortValue"
        :options="sortOptions"
        class="browse-view__sort"
      />
    </Teleport>

    <div v-if="excludedProfilesCount > 0" class="browse-view__notice">
      {{ excludedProfilesCount }} profile<span v-if="excludedProfilesCount !== 1">s</span> excluded.
    </div>

    <div v-if="loading" class="browse-view__state browse-view__state--loading">
      Loading notes…
    </div>

    <div v-else-if="error" class="browse-view__state browse-view__state--error">
      {{ error }}
    </div>

    <div v-else-if="notes.length === 0" class="browse-view__state browse-view__state--empty">
      <NyxIcon name="search" :size="28" />
      <span class="browse-view__empty-title">{{ emptyTitle }}</span>
      <span class="browse-view__empty-message">{{ emptyMessage }}</span>
    </div>

    <main v-else class="browse-view__body">
      <NyxGrid :title="title" :mode="NyxGridMode.Masonry" :columns="5">
        <NoteCard
          v-for="note in notes"
          :key="`${note.profile_id}:${note.vault_id}:${note.note_id}`"
          :note="note"
        />
      </NyxGrid>
    </main>
  </div>
</template>

<style scoped>
.browse-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow: hidden;
}

.browse-view__header {
  height: 64px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0 1.5rem;
  box-shadow: 0 1px 0 0 var(--nyx-c-divider);
}

.browse-view__title {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--nyx-c-text-2);
}

.browse-view__sort {
  width: 240px;
  max-width: 100%;
}

.browse-view__notice {
  margin: 1rem 1.5rem 0;
  padding: 0.75rem 1rem;
  border-radius: var(--nyx-radius-lg);
  background: rgba(73, 67, 95, 0.22);
  color: var(--nyx-c-text-2);
  font-size: 0.75rem;
}

.browse-view__body,
.browse-view__state {
  flex: 1;
  overflow: auto;
  padding: 2rem 1.5rem;
}

.browse-view__state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  color: var(--nyx-c-text-3);
  text-align: center;
}

.browse-view__state--error {
  color: var(--nyx-c-danger, #ff8f8f);
}

.browse-view__empty-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--nyx-c-text-2);
}

.browse-view__empty-message {
  font-size: 0.8125rem;
}
</style>
