<script setup lang="ts">
import { computed, reactive, watch } from 'vue'
import { NyxButton, NyxForm, NyxFormField, NyxInput, NyxModal, NyxSelect } from 'nyx-kit/components'
import { NyxInputType, NyxTheme, NyxVariant } from 'nyx-kit/types'
import type { ManagedUserSummary, ServerRole } from '@/shared/types'

const props = defineProps<{
  open: boolean
  user?: ManagedUserSummary | null
  saving?: boolean
  currentUserId?: string | null
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
  submit: [payload: { username: string; email: string; display_name: string; role: ServerRole; password?: string }]
}>()

const form = reactive({
  username: '',
  email: '',
  display_name: '',
  role: 'user' as ServerRole,
  password: '',
})

const isEdit = computed(() => Boolean(props.user))
const roleOptions = [
  { label: 'User', value: 'user' },
  { label: 'Admin', value: 'admin' },
]

watch(
  () => [props.open, props.user] as const,
  () => {
    form.username = props.user?.username ?? ''
    form.email = props.user?.email ?? ''
    form.display_name = props.user?.display_name ?? ''
    form.role = props.user?.role ?? 'user'
    form.password = ''
  },
  { immediate: true }
)

const roleDisabled = computed(() => props.user?.id === props.currentUserId)

function submit() {
  emit('submit', {
    username: form.username,
    email: form.email,
    display_name: form.display_name,
    role: form.role,
    password: form.password || undefined,
  })
}
</script>

<template>
  <NyxModal :model-value="open" :title="isEdit ? 'Edit User' : 'Add User'" @update:model-value="emit('update:open', $event)">
    <NyxForm class="create-edit-user" @submit.prevent="submit">
      <NyxFormField label="Username">
        <template #default="{ id }">
          <NyxInput :id="id" v-model="form.username" :variant="NyxVariant.Soft" :disabled="isEdit" />
        </template>
      </NyxFormField>

      <NyxFormField label="Email">
        <template #default="{ id }">
          <NyxInput :id="id" v-model="form.email" :variant="NyxVariant.Soft" />
        </template>
      </NyxFormField>

      <NyxFormField label="Display Name">
        <template #default="{ id }">
          <NyxInput :id="id" v-model="form.display_name" :variant="NyxVariant.Soft" />
        </template>
      </NyxFormField>

      <NyxFormField label="Role">
        <template #default="{ id }">
          <NyxSelect :id="id" v-model="form.role" :options="roleOptions" :disabled="roleDisabled" :variant="NyxVariant.Soft" />
        </template>
      </NyxFormField>

      <NyxFormField :label="isEdit ? 'New Password' : 'Password'" hint="At least 12 characters and 3 of 4 categories: lowercase, uppercase, digit, symbol.">
        <template #default="{ id }">
          <NyxInput :id="id" v-model="form.password" :type="NyxInputType.Password" :variant="NyxVariant.Soft" />
        </template>
      </NyxFormField>

      <div class="create-edit-user__actions">
        <NyxButton type="button" @click="emit('update:open', false)">Cancel</NyxButton>
        <NyxButton :gradient="true" type="submit" :disabled="saving" :theme="NyxTheme.Success" :variant="NyxVariant.Soft">{{ isEdit ? 'Save' : 'Create' }}</NyxButton>
      </div>
    </NyxForm>
  </NyxModal>
</template>

<style scoped>
.create-edit-user {
  display: grid;
  gap: 1rem;
}

.create-edit-user__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}
</style>
