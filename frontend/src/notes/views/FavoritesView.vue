<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import type { NyxSelectOption } from 'nyx-kit/types'
import { GlobalNoteBrowseView } from '@/notes/components'
import { useNoteBrowsingStore } from '@/notes/stores'

const browsingStore = useNoteBrowsingStore()
const {
  favoriteResults,
  favoriteRefs,
  favoritesSortMode,
  favoritesLoading,
  favoritesError,
  favoritesExcludedProfilesCount,
} = storeToRefs(browsingStore)
const { loadFavorites, setFavoritesSortMode } = browsingStore

const sortOptions: NyxSelectOption[] = [
  { label: 'Recent', value: 'recent' },
  { label: 'Grouped by origin', value: 'grouped' },
]

onMounted(async () => {
  await loadFavorites()
})

watch(favoriteRefs, async () => {
  await loadFavorites()
}, { deep: true })
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
    @update:sort-model-value="setFavoritesSortMode($event as 'recent' | 'grouped')"
  />
</template>
