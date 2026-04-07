<script setup lang="ts">
import { NyxInput, NyxBadge } from 'nyx-kit/components'
import { NyxSize, NyxVariant, NyxTheme } from 'nyx-kit/types'
import type { NotePermission } from '@/shared/types'

const props = defineProps<{
  title: string
  tags: string[]
  permission: NotePermission
  saving: boolean
  readonly: boolean
}>()

const emit = defineEmits<{
  'update:title': [value: string]
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
        <NyxBadge v-for="tag in props.tags" :key="tag" :theme="NyxTheme.Primary" :variant="NyxVariant.Soft">{{ tag }}</NyxBadge>
      </div>

      <span class="note-toolbar__status">
        <span v-if="props.saving" class="note-toolbar__saving">Saving…</span>
      </span>
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

.note-toolbar__status {
  margin-left: auto;
}

.note-toolbar__saving {
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--nyx-c-text-3);
}

</style>
