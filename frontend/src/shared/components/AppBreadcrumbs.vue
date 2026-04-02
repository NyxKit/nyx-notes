<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxBreadcrumbs } from 'nyx-kit/components'
import type { NyxBreadcrumb } from 'nyx-kit/types'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { RouteName } from '@/shared/types'
import { DEFAULT_NOTE_TITLE } from '@/notes'

const route = useRoute()
const { activeVault } = storeToRefs(useVaultStore())
const { activeNote } = storeToRefs(useNotesStore())

const breadcrumbs = computed((): NyxBreadcrumb[] => {
  const breadcrumbs: NyxBreadcrumb[] = [{
    label: 'Home',
    route: { name: RouteName.Home },
  }]

  const genericRoutes = [
    { name: RouteName.Favorites, label: 'Favorites' },
    { name: RouteName.Search, label: 'Search' },
  ]

  if (genericRoutes.some((r) => r.name === route.name)) {
    breadcrumbs.push({
      label: genericRoutes.find((r) => r.name === route.name)?.label ?? '',
      route: { name: route.name },
    })
    return breadcrumbs
  }

  if ([RouteName.Vault, RouteName.Note].includes(route.name as RouteName)) {
    const vaultId = String(route.params.vault_id ?? '')
    breadcrumbs.push({
      label: activeVault.value?.name ?? '',
      route: { name: RouteName.Vault, params: { vault_id: vaultId } },
    })
  }

  if (route.name === RouteName.Note) {
    const vaultId = String(route.params.vault_id ?? '')
    const noteId = String(route.params.id ?? '')
    breadcrumbs.push({
      label: activeNote.value?.meta.title || DEFAULT_NOTE_TITLE,
      route: { name: RouteName.Note, params: { vault_id: vaultId, id: noteId } },
    })
  }

  return breadcrumbs
})
</script>

<template>
  <NyxBreadcrumbs :items="breadcrumbs" />
</template>
