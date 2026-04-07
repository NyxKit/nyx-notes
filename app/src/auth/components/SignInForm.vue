<script setup lang="ts">
import { reactive } from 'vue'
import { NyxButton, NyxForm, NyxFormField, NyxInput } from 'nyx-kit/components'
import { NyxInputType, NyxVariant } from 'nyx-kit/types'

const props = withDefaults(defineProps<{
  username?: string
  loading?: boolean
}>(), {
  username: '',
  loading: false,
})

const emit = defineEmits<{
  submit: [data: { username: string; password: string }]
}>()

const form = reactive({
  username: props.username,
  password: '',
})

function handleSubmit() {
  emit('submit', { username: form.username, password: form.password })
}
</script>

<template>
  <NyxForm @submit="handleSubmit">
    <NyxFormField label="Username">
      <NyxInput
        v-model="form.username"
        :variant="NyxVariant.Soft"
        placeholder="username"
        required
        :disabled="loading"
      />
    </NyxFormField>

    <NyxFormField label="Password">
      <NyxInput
        v-model="form.password"
        :variant="NyxVariant.Soft"
        :type="NyxInputType.Password"
        required
        :disabled="loading"
      />
    </NyxFormField>

    <NyxButton
      type="submit"
      :variant="NyxVariant.Soft"
      :loading="loading"
    >
      Sign In
    </NyxButton>
  </NyxForm>
</template>
