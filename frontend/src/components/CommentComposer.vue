<script setup lang="ts">
import { ref } from 'vue'
import { NyxButton, NyxTextarea, NyxForm, NyxFormField } from 'nyx-kit/components'
import { NyxVariant } from 'nyx-kit/types'

const props = defineProps<{
  quotedText?: string
  placeholder?: string
  submitting?: boolean
}>()

const emit = defineEmits<{
  submit: [body: string]
  cancel: []
}>()

const body = ref('')

function onSubmit() {
  const trimmed = body.value.trim()
  if (!trimmed) return
  emit('submit', trimmed)
  body.value = ''
}
</script>

<template>
  <div class="composer">
    <div v-if="props.quotedText" class="composer__quote">
      {{ props.quotedText }}
    </div>
    <NyxForm @submit="onSubmit">
      <NyxFormField>
        <template #default="{ id }">
          <NyxTextarea
            :id="id"
            v-model="body"
            :placeholder="props.placeholder ?? 'Add a comment…'"
            :rows="3"
            @keydown.ctrl.enter="onSubmit"
            @keydown.meta.enter="onSubmit"
          />
        </template>
      </NyxFormField>
      <div class="composer__actions">
        <NyxButton @click="emit('cancel')">
          Cancel
        </NyxButton>
        <NyxButton
          type="submit"
          :disabled="!body.trim() || props.submitting"
        >
          {{ props.submitting ? 'Posting…' : 'Comment' }}
        </NyxButton>
      </div>
    </NyxForm>
  </div>
</template>

<style scoped>
.composer {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.composer__quote {
  font-size: 0.8125rem;
  border-left: 3px solid var(--nyx-color-accent, #6366f1);
  padding: 0.25rem 0.625rem;
  color: var(--nyx-color-muted, #718096);
  background: var(--nyx-color-surface-raised, #f7fafc);
  border-radius: 0 0.25rem 0.25rem 0;
  white-space: pre-wrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
}

.composer__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}
</style>
