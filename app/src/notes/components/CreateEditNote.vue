<script setup lang="ts">
import { NyxEditor, NyxInput, NyxFormField } from 'nyx-kit/components'
import { NyxEditorFormat, NyxEditorMode, NyxEditorToolbar, NyxSize, NyxVariant } from 'nyx-kit/types'

defineProps<{
  title: string
  content: string
  readonly?: boolean
  saving?: boolean
  titleLabel?: string
  contentPlaceholder?: string
}>()

const emit = defineEmits<{
  'update:title': [value: string]
  'update:content': [value: string]
}>()
</script>

<template>
  <div class="create-edit-note">
    <NyxFormField :label="titleLabel ?? 'Title'">
      <template #default="{ id }">
        <NyxInput
          :id="id"
          :model-value="title"
          :variant="NyxVariant.Text"
          :size="NyxSize.XLarge"
          :readonly="readonly"
          placeholder="Untitled"
          @update:model-value="emit('update:title', $event)"
        />
      </template>
    </NyxFormField>

    <NyxEditor
      class="create-edit-note__body"
      :model-value="content"
      :source="false"
      :variant="NyxVariant.Text"
      :toolbar="NyxEditorToolbar.Full"
      :format="NyxEditorFormat.Markdown"
      :mode="NyxEditorMode.Zen"
      :disabled="readonly"
      :placeholder="contentPlaceholder ?? 'Start writing…'"
      @change="emit('update:content', $event)"
    />
  </div>
</template>

<style scoped>
.create-edit-note {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  min-height: 0;
}

.create-edit-note__body {
  min-height: 280px;
}
</style>
