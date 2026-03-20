<script setup lang="ts">
import { NyxInput, NyxButton } from 'nyx-kit/components'
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
  isCommentsOpen: boolean
}>()

const emit = defineEmits<{
  'update:title': [value: string],
  'toggle:source': []
  'toggle:comments': []
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
      <span v-if="props.tags.length" class="note-toolbar__tags">
        <span v-for="tag in props.tags" :key="tag" class="note-toolbar__tag">{{ tag }}</span>
      </span>

      <span class="note-toolbar__status">
        <span v-if="props.saving" class="note-toolbar__status--saving">Saving…</span>
      </span>
    </div>

    <NyxButton
      v-if="props.isAuthor"
      class="note-toolbar__source-view-toggle"
      :variant="NyxVariant.Ghost"
      :size="NyxSize.Small"
      @click="emit('toggle:source')"
    >
      Toggle source view
    </NyxButton>

    <NyxButton
      class="note-toolbar__toggle-comments"
      :variant="NyxVariant.Ghost"
      :size="NyxSize.Small"
      @click="emit('toggle:comments')"
    >
      {{ props.isCommentsOpen ? 'Hide comments' : 'Show comments' }}
    </NyxButton>
  </div>
</template>

<style scoped>
.note-toolbar {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
  border-bottom: 1px solid var(--nyx-color-border, #e2e8f0);
}

.note-toolbar__title {
  --nyx-font-size-input: 2.5rem;
  flex: 1;
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

.note-toolbar__source-view-toggle {
  margin: 1.5rem;
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
