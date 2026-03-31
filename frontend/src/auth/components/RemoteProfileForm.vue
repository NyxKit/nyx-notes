<script setup lang="ts">
import { reactive, watch } from 'vue'
import { NyxButton, NyxForm, NyxFormField, NyxInput } from 'nyx-kit/components'
import { NyxInputType } from 'nyx-kit/types'
import type { RemoteProfileDraft } from '@/shared/types'

const props = withDefaults(defineProps<{
  initialValue?: Partial<RemoteProfileDraft>
  submitLabel?: string
  includeDisplayName?: boolean
  loading?: boolean
}>(), {
  initialValue: () => ({}),
  submitLabel: 'Save Profile',
  includeDisplayName: true,
  loading: false,
})

const emit = defineEmits<{
  submit: [draft: RemoteProfileDraft]
}>()

const form = reactive<RemoteProfileDraft>({
  display_name: '',
  server_url: '',
  username: '',
  password: '',
})

watch(() => props.initialValue, (value) => {
  form.display_name = value.display_name ?? ''
  form.server_url = value.server_url ?? ''
  form.username = value.username ?? ''
  form.password = value.password ?? ''
}, { immediate: true, deep: true })

function handleSubmit() {
  emit('submit', { ...form })
}
</script>

<template>
  <NyxForm @submit="handleSubmit">
    <NyxFormField v-if="includeDisplayName" label="Profile Name">
      <NyxInput v-model="form.display_name" placeholder="Work NAS" />
    </NyxFormField>

    <NyxFormField label="Server URL">
      <NyxInput v-model="form.server_url" placeholder="https://notes.example.com" required />
    </NyxFormField>

    <NyxFormField label="Username">
      <NyxInput v-model="form.username" placeholder="alice" required />
    </NyxFormField>

    <NyxFormField label="Password">
      <NyxInput v-model="form.password" :type="NyxInputType.Password" placeholder="••••••••" required />
    </NyxFormField>

    <NyxButton type="submit" :disabled="loading">
      {{ loading ? 'Working…' : submitLabel }}
    </NyxButton>
  </NyxForm>
</template>
