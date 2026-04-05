<script setup lang="ts">
import { ref } from 'vue'
import { NyxButton, NyxForm, NyxFormField, NyxInput } from 'nyx-kit/components'
import { NyxInputType, NyxVariant } from 'nyx-kit/types'
import { setup, type SetupInput } from '@/auth/api'
import type { InitialSetupState } from '@/auth/types/profileSetup'

const emit = defineEmits<{
  complete: []
}>()

defineProps<{
  loading?: boolean
}>()

const state = ref<InitialSetupState>('uninitialized')
const error = ref<string | null>(null)
const passwordError = ref<string | null>(null)

const input = ref<SetupInput>({
  username: '',
  email: '',
  display_name: '',
  password: '',
  confirm_password: '',
})

function validatePasswords() {
  passwordError.value = null
  if (input.value.password !== input.value.confirm_password) {
    passwordError.value = 'Passwords do not match'
    return false
  }
  if (input.value.password.length < 12) {
    passwordError.value = 'Password must be at least 12 characters'
    return false
  }
  const hasLower = /[a-z]/.test(input.value.password)
  const hasUpper = /[A-Z]/.test(input.value.password)
  const hasDigit = /[0-9]/.test(input.value.password)
  const categories = [hasLower, hasUpper, hasDigit].filter(Boolean).length
  if (categories < 3) {
    passwordError.value = 'Password must contain at least 3 of: lowercase, uppercase, and numbers'
    return false
  }
  return true
}

async function submitSetup() {
  error.value = null
  if (!validatePasswords()) {
    return
  }
  
  state.value = 'initializing'

  try {
    await setup(input.value)
    state.value = 'initialized'
    emit('complete')
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : 'Setup failed'
    state.value = 'uninitialized'
  }
}
</script>

<template>
  <NyxForm class="setup-form" @submit.prevent="submitSetup">
    <NyxFormField label="Username">
      <NyxInput
        v-model="input.username"
        :variant="NyxVariant.Soft"
        placeholder="admin"
        required
        :disabled="loading || state === 'initializing'"
      />
    </NyxFormField>

    <NyxFormField label="Display Name">
      <NyxInput
        v-model="input.display_name"
        :variant="NyxVariant.Soft"
        placeholder="Admin"
        required
        :disabled="loading || state === 'initializing'"
      />
    </NyxFormField>

    <NyxFormField label="Email">
      <NyxInput
        v-model="input.email"
        :variant="NyxVariant.Soft"
        :type="NyxInputType.Email"
        placeholder="admin@example.com"
        required
        :disabled="loading || state === 'initializing'"
      />
    </NyxFormField>

    <NyxFormField label="Password">
      <NyxInput
        v-model="input.password"
        :variant="NyxVariant.Soft"
        :type="NyxInputType.Password"
        required
        :disabled="loading || state === 'initializing'"
      />
    </NyxFormField>

    <NyxFormField label="Confirm Password">
      <NyxInput
        v-model="input.confirm_password"
        :variant="NyxVariant.Soft"
        :type="NyxInputType.Password"
        required
        :disabled="loading || state === 'initializing'"
      />
    </NyxFormField>

    <NyxButton
      class="setup-form__button"
      type="submit"
      :variant="NyxVariant.Soft"
      :loading="loading || state === 'initializing'"
      :disabled="state === 'initializing'"
    >
      Create Account
    </NyxButton>

    <p v-if="passwordError" class="setup-form__error">{{ passwordError }}</p>
    <p v-if="error" class="setup-form__error">{{ error }}</p>
  </NyxForm>
</template>

<style scoped>
.setup-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.setup-form form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.setup-form__error {
  color: var(--nyx-color-danger, #e53e3e);
  font-size: 0.875rem;
}

.setup-form__button {
  margin-top: 1rem;
}
</style>
