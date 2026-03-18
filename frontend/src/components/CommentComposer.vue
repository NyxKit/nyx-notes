<script setup lang="ts">
import { ref } from 'vue'

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
    <textarea
      v-model="body"
      class="composer__input"
      :placeholder="props.placeholder ?? 'Add a comment…'"
      rows="3"
      @keydown.ctrl.enter="onSubmit"
      @keydown.meta.enter="onSubmit"
    />
    <div class="composer__actions">
      <button
        class="composer__btn composer__btn--cancel"
        type="button"
        @click="emit('cancel')"
      >
        Cancel
      </button>
      <button
        class="composer__btn composer__btn--submit"
        type="button"
        :disabled="!body.trim() || props.submitting"
        @click="onSubmit"
      >
        {{ props.submitting ? 'Posting…' : 'Comment' }}
      </button>
    </div>
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

.composer__input {
  resize: vertical;
  border: 1px solid var(--nyx-color-border, #e2e8f0);
  border-radius: 0.375rem;
  padding: 0.5rem 0.625rem;
  font-family: inherit;
  font-size: 0.875rem;
  line-height: 1.5;
  background: transparent;
  color: inherit;
  min-height: 72px;
}

.composer__input:focus {
  outline: none;
  border-color: var(--nyx-color-accent, #6366f1);
}

.composer__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}

.composer__btn {
  font-size: 0.8125rem;
  padding: 0.3125rem 0.75rem;
  border-radius: 0.375rem;
  cursor: pointer;
  border: 1px solid transparent;
  font-family: inherit;
}

.composer__btn--cancel {
  background: transparent;
  border-color: var(--nyx-color-border, #e2e8f0);
  color: var(--nyx-color-muted, #718096);
}

.composer__btn--submit {
  background: var(--nyx-color-accent, #6366f1);
  color: #fff;
}

.composer__btn--submit:disabled {
  opacity: 0.5;
  cursor: default;
}
</style>
