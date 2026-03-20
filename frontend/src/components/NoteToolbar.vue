<script setup lang="ts">
import { NyxInput } from 'nyx-kit/components'
import { NyxSize, NyxVariant } from 'nyx-kit/types'
import type { NotePermission } from '@/types'

const props = defineProps<{
  title: string
  tags: string[]
  permission: NotePermission
  saving: boolean
  readonly: boolean
  isAuthor: boolean
  isSourceView: boolean
}>()

const emit = defineEmits<{
  'update:title': [value: string]
  'toggle:source': []
}>()
</script>

<template>
  <div class="note-toolbar">
    <NyxInput
      class="note-toolbar__title"
      :model-value="props.title"
      :variant="NyxVariant.Text"
      :size="NyxSize.XLarge"
      :readonly="props.readonly"
      placeholder="Untitled"
      @update:model-value="emit('update:title', $event)"
    />

    <div class="note-toolbar__meta">
      <div v-if="props.tags.length" class="note-toolbar__tags">
        <span v-for="tag in props.tags" :key="tag" class="note-toolbar__tag">{{ tag }}</span>
      </div>

      <span class="note-toolbar__status">
        <span v-if="props.saving" class="note-toolbar__saving">Saving…</span>
      </span>

      <button
        v-if="props.isAuthor"
        class="note-toolbar__source-btn"
        :class="{ 'note-toolbar__source-btn--active': props.isSourceView }"
        :title="props.isSourceView ? 'Exit source view' : 'Source view'"
        @click="emit('toggle:source')"
      >
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
          <path d="M4 3L1 7l3 4M10 3l3 4-3 4M8 2l-2 10" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.note-toolbar {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 3rem 0 0;
  max-width: 768px;
  width: 100%;
  margin: 0 auto;
}

.note-toolbar__title {
  --nyx-font-size-input: 2.75rem;
}

.note-toolbar__meta {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.note-toolbar__tags {
  display: flex;
  gap: 0.375rem;
  flex-wrap: wrap;
}

.note-toolbar__tag {
  border: 1px solid rgba(71, 71, 77, 0.35);
  border-radius: 9999px;
  padding: 0.125rem 0.625rem;
  font-size: 0.625rem;
  font-family: 'Inter', sans-serif;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--nyx-c-text-2);
  background: transparent;
}

.note-toolbar__status {
  margin-left: auto;
}

.note-toolbar__saving {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

.note-toolbar__source-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--nyx-c-text-3);
  padding: 0.25rem;
  border-radius: var(--nyx-radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.2s, background 0.2s;
  line-height: 0;
}

.note-toolbar__source-btn:hover {
  color: var(--nyx-c-text-2);
  background: var(--nyx-c-bg-mute);
}

.note-toolbar__source-btn--active {
  color: var(--nyx-c-primary);
}
</style>
