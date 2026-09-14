<script setup lang="ts">
import { storeToRefs } from 'pinia'
import type { NyxSelectOption } from 'nyx-kit/types'
import { GlobalNoteBrowseView } from '@/notes/components'
import { useNoteBrowsingStore } from '@/notes/stores'
import { GlobalBrowseSortMode } from '@/shared/types'

const browsingStore = useNoteBrowsingStore()
const {
  favoriteResults,
  favoritesSortMode,
  favoritesLoading,
  favoritesError,
  favoritesExcludedProfilesCount,
} = storeToRefs(browsingStore)
const { setFavoritesSortMode } = browsingStore

const sortOptions: NyxSelectOption[] = [
  { label: 'Recent', value: GlobalBrowseSortMode.Recent },
  { label: 'Grouped by origin', value: GlobalBrowseSortMode.Grouped },
]

</script>

<template>
  <GlobalNoteBrowseView
    :notes="favoriteResults"
    :loading="favoritesLoading"
    :error="favoritesError"
    :excluded-profiles-count="favoritesExcludedProfilesCount"
    :sort-model-value="favoritesSortMode"
    :sort-options="sortOptions"
    empty-title="No favorite notes yet"
    empty-message="Favorite notes from any reachable profile or vault to collect them here."
    @update:sort-model-value="setFavoritesSortMode($event as Exclude<GlobalBrowseSortMode, GlobalBrowseSortMode.BestMatch>)"
  />
</template>
