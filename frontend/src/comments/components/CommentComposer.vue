<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'
import { NyxForm, NyxFormField, NyxInput } from 'nyx-kit/components'

const props = defineProps<{
  quotedText?: string
  placeholder?: string
  submitting?: boolean
  autofocus?: boolean
}>()

const emit = defineEmits<{
  submit: [body: string]
  cancel: []
}>()

const body = ref('')
const inputHostRef = ref<HTMLElement | null>(null)

function onSubmit() {
  const trimmed = body.value.trim()
  if (!trimmed) return
  emit('submit', trimmed)
  body.value = ''
}

function onEscape() {
  body.value = ''
  emit('cancel')
}

async function focusInput() {
  await nextTick()
  requestAnimationFrame(() => {
    const input = inputHostRef.value?.querySelector('input') as HTMLInputElement | null
    input?.focus()
    input?.select()
  })
}

watch(
  () => props.autofocus,
  async (autofocus) => {
    if (!autofocus) return
    await focusInput()
  },
  { immediate: true }
)
</script>

<template>
  <div class="composer">
    <div v-if="props.quotedText" class="composer__quote">
      {{ props.quotedText }}
    </div>
    <NyxForm class="composer__form" @submit.prevent="onSubmit">
      <div ref="inputHostRef" class="composer__field">
        <NyxFormField>
          <template #default="{ id }">
            <NyxInput
              :id="id"
              v-model="body"
              class="composer__input"
              :autofocus="props.autofocus"
              :placeholder="props.placeholder ?? 'Add a comment…'"
              @keydown.esc="onEscape"
            />
          </template>
        </NyxFormField>
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

.composer__form,
.composer__field,
.composer__input {
  width: 100%;
}
</style>
