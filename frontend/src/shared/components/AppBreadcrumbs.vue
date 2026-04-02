<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxBreadcrumbs } from 'nyx-kit/components'
import type { NyxBreadcrumb } from 'nyx-kit/types'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import VaultIcon from '@/vaults/components/VaultIcon.vue'
import { RouteName } from '@/shared/types'

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
    { name: RouteName.Drafts, label: 'Drafts' },
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
    breadcrumbs.push({
      label: activeVault.value?.name ?? '',
      route: { name: RouteName.Vault, params: { vault_id: activeVault.value?.id ?? '' } },
    })
  }

  if (route.name === RouteName.Note) {
    breadcrumbs.push({
      label: activeNote.value?.meta.title ?? '',
      route: { name: RouteName.Note, params: { vault_id: activeVault.value?.id ?? '', id: activeNote.value?.meta.id ?? '' } },
    })
  }

  return breadcrumbs
})

function isVaultBreadcrumb(item: NyxBreadcrumb) {
  return typeof item.route === 'object' && item.route !== null && 'name' in item.route && item.route.name === RouteName.Vault
}

function vaultBreadcrumbIcon() {
  return activeVault.value?.icon || 'folder'
}
</script>

<template>
  <NyxBreadcrumbs :items="breadcrumbs" />
</template>

<style scoped>
.layout-breadcrumbs__vault-item {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
}

.layout-breadcrumbs__vault-icon {
  opacity: 0.75;
}
</style>
