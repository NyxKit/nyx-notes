<script setup lang="ts">
import { NyxButton, NyxIcon, NyxTable, NyxTableCell } from 'nyx-kit/components'
import type { ManagedUserSummary } from '@/shared/types'

defineProps<{
  users: ManagedUserSummary[]
  deletingId?: string | null
}>()

const emit = defineEmits<{
  edit: [user: ManagedUserSummary]
  delete: [user: ManagedUserSummary]
}>()
</script>

<template>
  <NyxTable
    class="users-table"
    :model-value="users"
    item-key="id"
    :column-titles="['Username', 'Email', 'Name', 'Role']"
    :grid-template-columns="'1fr 1.3fr 1.2fr 0.8fr 1.2fr'"
  >
    <template #default="{ item }">
      <NyxTableCell>{{ item.username }}</NyxTableCell>
      <NyxTableCell>{{ item.email }}</NyxTableCell>
      <NyxTableCell>{{ item.display_name }}</NyxTableCell>
      <NyxTableCell>{{ item.role }}</NyxTableCell>
    </template>

    <template #actions="{ item }">
      <div class="users-table__actions">
        <NyxButton :disabled="!item.can_edit" @click="emit('edit', item as ManagedUserSummary)">
          <NyxIcon name="pencil" :size="16" />
          Edit
        </NyxButton>
        <NyxButton :disabled="!item.can_delete || deletingId === item.id" @click="emit('delete', item as ManagedUserSummary)">
          <NyxIcon name="trash-2" :size="16" />
          Delete
        </NyxButton>
      </div>
    </template>
  </NyxTable>
</template>

<style scoped>
.users-table__actions {
  display: flex;
  gap: 0.5rem;
}
</style>
