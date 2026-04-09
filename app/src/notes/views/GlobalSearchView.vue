<script setup lang="ts">
import { computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import type { NyxSelectOption } from 'nyx-kit/types'
import { GlobalNoteBrowseView } from '@/notes/components'
import { useNoteBrowsingStore } from '@/notes/stores'
import { GlobalBrowseSortMode } from '@/shared/types'

const route = useRoute()
const browsingStore = useNoteBrowsingStore()
const {
  searchResults,
  searchSortMode,
  favoriteRefs,
  searchLoading,
  searchError,
  searchExcludedProfilesCount,
} = storeToRefs(browsingStore)
const { loadSearch, setSearchSortMode } = browsingStore

const sortOptions: NyxSelectOption[] = [
  { label: 'Best match', value: GlobalBrowseSortMode.BestMatch },
  { label: 'Recent', value: GlobalBrowseSortMode.Recent },
  { label: 'Grouped by origin', value: GlobalBrowseSortMode.Grouped },
]

const sortMode = computed({
  get: () => searchSortMode.value,
  set: (value: string) => setSearchSortMode(value as GlobalBrowseSortMode),
})

const query = computed(() => String(route.query.q ?? ''))

watch(query, async (value) => {
  await loadSearch(value)
}, { immediate: true })

watch(favoriteRefs, async () => {
  await loadSearch(query.value)
}, { deep: true })
</script>

<template>
  <GlobalNoteBrowseView
    :notes="searchResults"
    :loading="searchLoading"
    :error="searchError"
    :excluded-profiles-count="searchExcludedProfilesCount"
    :sort-model-value="sortMode"
    :sort-options="sortOptions"
    :empty-title="query ? 'No matching notes' : 'Start typing to search'"
    :empty-message="query ? 'Try another phrase, tag, or title.' : 'Use the sidebar search bar to search across all reachable profiles and vaults.'"
    @update:sort-model-value="setSearchSortMode($event as GlobalBrowseSortMode)"
  />
</template>
