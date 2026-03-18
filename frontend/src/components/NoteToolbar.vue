<script setup lang="ts">
import type { NotePermission } from '@/types'

const props = defineProps<{
  title: string
  tags: string[]
  permission: NotePermission
  saving: boolean
  readonly: boolean
  isAuthor: boolean
}>()

const emit = defineEmits<{
  'update:title': [value: string]
  'update:permission': [value: NotePermission]
}>()

const permissionOptions: { label: string; value: NotePermission }[] = [
  { label: 'Restricted', value: 'restricted' },
  { label: 'Comment', value: 'comment' },
  { label: 'Edit', value: 'edit' },
]
</script>

<template>
  <div class="note-toolbar">
    <input
      class="note-toolbar__title"
      type="text"
      :value="props.title"
      :disabled="props.readonly"
      placeholder="Untitled"
      @input="emit('update:title', ($event.target as HTMLInputElement).value)"
    />

    <div class="note-toolbar__meta">
      <span v-if="props.tags.length" class="note-toolbar__tags">
        <span v-for="tag in props.tags" :key="tag" class="note-toolbar__tag">{{ tag }}</span>
      </span>

      <select
        v-if="props.isAuthor"
        class="note-toolbar__permission"
        :value="props.permission"
        @change="emit('update:permission', ($event.target as HTMLSelectElement).value as NotePermission)"
      >
        <option v-for="opt in permissionOptions" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
      <span v-else class="note-toolbar__permission-badge">{{ props.permission }}</span>

      <span class="note-toolbar__status">
        <span v-if="props.saving" class="note-toolbar__status--saving">Saving…</span>
      </span>
    </div>
  </div>
</template>

<style scoped>
.note-toolbar {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1.25rem 1.5rem 0.75rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.note-toolbar__title {
  font-size: 1.5rem;
  font-weight: 600;
  border: none;
  outline: none;
  background: transparent;
  color: inherit;
  width: 100%;
  font-family: inherit;
}

.note-toolbar__title:disabled {
  opacity: 0.7;
  cursor: default;
}

.note-toolbar__meta {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-size: 0.8125rem;
  color: var(--nyx-color-muted, #718096);
}

.note-toolbar__tags {
  display: flex;
  gap: 0.375rem;
  flex-wrap: wrap;
}

.note-toolbar__tag {
  background: var(--nyx-color-surface-raised, #f7fafc);
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 9999px;
  padding: 0.125rem 0.625rem;
  font-size: 0.75rem;
}

.note-toolbar__permission {
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 0.25rem;
  padding: 0.125rem 0.375rem;
  font-size: 0.75rem;
  background: transparent;
  color: inherit;
  cursor: pointer;
}

.note-toolbar__permission-badge {
  font-size: 0.75rem;
  opacity: 0.6;
}

.note-toolbar__status {
  margin-left: auto;
}

.note-toolbar__status--saving {
  font-size: 0.75rem;
  opacity: 0.6;
  font-style: italic;
}
</style>
