<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NyxBreadcrumbs } from 'nyx-kit/components'
import type { NyxBreadcrumb } from 'nyx-kit/types'
import { useAuth } from '@/auth/composables'
import { noteCrumbRouteFromParams, vaultCrumbRouteFromParams } from '@/shared/utils'
import { useVaultStore } from '@/vaults/stores'
import { useNotesStore } from '@/notes/stores'
import { RouteName } from '@/shared/types'
import { DEFAULT_NOTE_TITLE } from '@/notes'

const route = useRoute()
const { serverMetadata } = useAuth()
const { vaults } = storeToRefs(useVaultStore())
const { activeNote } = storeToRefs(useNotesStore())

const breadcrumbs = computed((): NyxBreadcrumb[] => {
  const breadcrumbs: NyxBreadcrumb[] = [{
    label: 'Nyx',
    route: { name: RouteName.Home },
  }]

  const serverSlug = route.params.server_slug as string | undefined
  const homeSlug = route.params.home_slug as string | undefined

  if (serverSlug) {
    breadcrumbs.push({
      label: serverMetadata.value?.slug === serverSlug ? serverMetadata.value.name : serverSlug,
      route: { path: `/${serverSlug}/vaults` },
    })
  }

  if (serverSlug && homeSlug) {
    breadcrumbs.push({
      label: homeSlug,
      route: { path: `/${serverSlug}/homes/${homeSlug}` },
    })
  }

  const genericRoutes = [
    { name: RouteName.Favorites, label: 'Favorites' },
    { name: RouteName.Search, label: 'Search' },
    { name: RouteName.Settings, label: 'Settings' },
  ]

  if (genericRoutes.some((r) => r.name === route.name)) {
    breadcrumbs.push({
      label: genericRoutes.find((r) => r.name === route.name)?.label ?? '',
      route: { name: route.name },
    })
    return breadcrumbs
  }

  if (route.name === RouteName.Vault || route.name === RouteName.Note) {
    const vaultId = String(route.params.vault_id ?? '')
    const vaultName = vaults.value.find(v => v.slug === vaultId)?.name ?? String(route.params.vault_id ?? '')
    breadcrumbs.push({
      label: vaultName,
      route: vaultCrumbRouteFromParams(
        serverSlug,
        homeSlug,
        vaultId,
      ),
    })
  }

  if (route.name === RouteName.Note && activeNote.value) {
    breadcrumbs.push({
      label: activeNote.value.meta.title || DEFAULT_NOTE_TITLE,
      route: noteCrumbRouteFromParams(
        serverSlug,
        homeSlug,
        activeNote.value.meta.vault_id,
        activeNote.value.meta.id,
      ),
    })
  }

  return breadcrumbs
})
</script>

<template>
  <NyxBreadcrumbs :items="breadcrumbs" />
</template>
