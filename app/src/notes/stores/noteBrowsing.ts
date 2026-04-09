import { computed, ref } from 'vue'
import { defineStore, acceptHMRUpdate } from 'pinia'
import { useGlobalNoteBrowsing } from '@/notes/composables'
import { GlobalBrowseSortMode } from '@/shared/types'
import type { BrowseNoteCardModel, FavoriteNoteRef, NoteMeta } from '@/shared/types'

const FAVORITES_KEY = 'nyx_favorites_v2'

function readFavoriteRefs(): FavoriteNoteRef[] {
  const raw = localStorage.getItem(FAVORITES_KEY)
  if (!raw) return []

  try {
    const parsed = JSON.parse(raw) as Array<FavoriteNoteRef | string>
    return parsed.map(entry => {
      if (typeof entry === 'string') {
        return {
          profile_id: '',
          vault_id: '',
          note_id: entry,
          created_at: new Date(0).toISOString(),
        }
      }

      return entry
    })
  } catch {
    return []
  }
}

function persistFavoriteRefs(favoriteRefs: FavoriteNoteRef[]) {
  localStorage.setItem(FAVORITES_KEY, JSON.stringify(favoriteRefs))
}

function sortRecent(notes: BrowseNoteCardModel[]) {
  return notes.slice().sort((left, right) =>
    Number(right.is_favorite) - Number(left.is_favorite)
    || new Date(right.updated_at).getTime() - new Date(left.updated_at).getTime()
  )
}

function sortGrouped(notes: BrowseNoteCardModel[]) {
  return notes.slice().sort((left, right) => {
    const favorite = Number(right.is_favorite) - Number(left.is_favorite)
    if (favorite !== 0) return favorite

    const server = left.server_label.localeCompare(right.server_label)
    if (server !== 0) return server

    const vault = left.vault_name.localeCompare(right.vault_name)
    if (vault !== 0) return vault

    return new Date(right.updated_at).getTime() - new Date(left.updated_at).getTime()
  })
}

function sortBestMatch(notes: BrowseNoteCardModel[]) {
  return notes.slice().sort((left, right) => {
    const favorite = Number(right.is_favorite) - Number(left.is_favorite)
    if (favorite !== 0) return favorite

    const score = (right.match_score ?? 0) - (left.match_score ?? 0)
    if (score !== 0) return score

    return new Date(right.updated_at).getTime() - new Date(left.updated_at).getTime()
  })
}

export const useNoteBrowsingStore = defineStore('noteBrowsing', () => {
  const browsing = useGlobalNoteBrowsing()

  const searchQuery = ref('')
  const searchSortMode = ref<GlobalBrowseSortMode>(GlobalBrowseSortMode.Recent)
  const favoritesSortMode = ref<Exclude<GlobalBrowseSortMode, GlobalBrowseSortMode.BestMatch>>(GlobalBrowseSortMode.Recent)
  const searchResultsRaw = ref<BrowseNoteCardModel[]>([])
  const favoriteResultsRaw = ref<BrowseNoteCardModel[]>([])
  const recentResultsRaw = ref<BrowseNoteCardModel[]>([])
  const favoriteRefs = ref<FavoriteNoteRef[]>(readFavoriteRefs())
  const searchLoading = ref(false)
  const favoritesLoading = ref(false)
  const recentLoading = ref(false)
  const searchError = ref<string | null>(null)
  const favoritesError = ref<string | null>(null)
  const recentError = ref<string | null>(null)
  const searchExcludedProfilesCount = ref(0)
  const favoritesExcludedProfilesCount = ref(0)
  const recentExcludedProfilesCount = ref(0)

  const searchResults = computed(() => {
    if (searchSortMode.value === GlobalBrowseSortMode.BestMatch) return sortBestMatch(searchResultsRaw.value)
    if (searchSortMode.value === GlobalBrowseSortMode.Grouped) return sortGrouped(searchResultsRaw.value)
    return sortRecent(searchResultsRaw.value)
  })

  const favoriteResults = computed(() => {
    if (favoritesSortMode.value === GlobalBrowseSortMode.Grouped) return sortGrouped(favoriteResultsRaw.value)
    return sortRecent(favoriteResultsRaw.value)
  })

  const recentResults = computed(() => (
    recentResultsRaw.value
      .slice()
      .sort((left, right) => new Date(right.updated_at).getTime() - new Date(left.updated_at).getTime())
  ))

  function isFavorite(profileId: string, vaultId: string, noteId: string) {
    return favoriteRefs.value.some(ref => (
      ref.note_id === noteId
      && (!ref.profile_id || ref.profile_id === profileId)
      && (!ref.vault_id || ref.vault_id === vaultId)
    ))
  }

  function toggleFavorite(profileId: string, vaultId: string, noteId: string) {
    const existingIndex = favoriteRefs.value.findIndex(ref => (
      ref.note_id === noteId
      && (!ref.profile_id || ref.profile_id === profileId)
      && (!ref.vault_id || ref.vault_id === vaultId)
    ))

    if (existingIndex !== -1) {
      favoriteRefs.value.splice(existingIndex, 1)
    } else {
      favoriteRefs.value.unshift({
        profile_id: profileId,
        vault_id: vaultId,
        note_id: noteId,
        created_at: new Date().toISOString(),
      })
    }

    searchResultsRaw.value = searchResultsRaw.value.map(note => (
      note.profile_id === profileId && note.vault_id === vaultId && note.note_id === noteId
        ? { ...note, is_favorite: existingIndex === -1 }
        : note
    ))

    if (existingIndex !== -1) {
      favoriteResultsRaw.value = favoriteResultsRaw.value.filter(note => !(
        note.profile_id === profileId && note.vault_id === vaultId && note.note_id === noteId
      ))
    }

    persistFavoriteRefs(favoriteRefs.value)
  }

  function toggleFavoriteForNote(profileId: string, note: NoteMeta) {
    toggleFavorite(profileId, note.vault_id, note.id)
  }

  async function loadSearch(query: string) {
    searchQuery.value = query
    searchLoading.value = true
    searchError.value = null

    try {
      const result = await browsing.loadSearchResults(query, favoriteRefs.value)
      searchResultsRaw.value = result.results
      searchExcludedProfilesCount.value = result.excluded_profiles_count
    } catch (error) {
      searchResultsRaw.value = []
      searchExcludedProfilesCount.value = 0
      searchError.value = error instanceof Error ? error.message : 'Unable to load search results'
    } finally {
      searchLoading.value = false
    }
  }

  async function loadFavorites() {
    favoritesLoading.value = true
    favoritesError.value = null

    try {
      const result = await browsing.loadFavoriteResults(favoriteRefs.value)
      favoriteResultsRaw.value = result.results
      favoritesExcludedProfilesCount.value = result.excluded_profiles_count
    } catch (error) {
      favoriteResultsRaw.value = []
      favoritesExcludedProfilesCount.value = 0
      favoritesError.value = error instanceof Error ? error.message : 'Unable to load favorites'
    } finally {
      favoritesLoading.value = false
    }
  }

  async function loadRecentNotes() {
    recentLoading.value = true
    recentError.value = null

    try {
      const result = await browsing.loadRecentResults(favoriteRefs.value)
      recentResultsRaw.value = result.results
      recentExcludedProfilesCount.value = result.excluded_profiles_count
    } catch (error) {
      recentResultsRaw.value = []
      recentExcludedProfilesCount.value = 0
      recentError.value = error instanceof Error ? error.message : 'Unable to load recent notes'
    } finally {
      recentLoading.value = false
    }
  }

  function setSearchSortMode(mode: GlobalBrowseSortMode) {
    searchSortMode.value = mode
  }

  function setFavoritesSortMode(mode: Exclude<GlobalBrowseSortMode, GlobalBrowseSortMode.BestMatch>) {
    favoritesSortMode.value = mode
  }

  function $reset() {
    searchQuery.value = ''
    searchSortMode.value = GlobalBrowseSortMode.Recent
    favoritesSortMode.value = GlobalBrowseSortMode.Recent
    searchResultsRaw.value = []
    favoriteResultsRaw.value = []
    favoriteRefs.value = readFavoriteRefs()
    recentResultsRaw.value = []
    searchLoading.value = false
    favoritesLoading.value = false
    recentLoading.value = false
    searchError.value = null
    favoritesError.value = null
    recentError.value = null
    searchExcludedProfilesCount.value = 0
    favoritesExcludedProfilesCount.value = 0
    recentExcludedProfilesCount.value = 0
  }

  return {
    searchQuery,
    searchSortMode,
    favoritesSortMode,
    searchResults,
    favoriteResults,
    recentResults,
    favoriteRefs,
    searchLoading,
    favoritesLoading,
    recentLoading,
    searchError,
    favoritesError,
    recentError,
    searchExcludedProfilesCount,
    favoritesExcludedProfilesCount,
    recentExcludedProfilesCount,
    isFavorite,
    toggleFavorite,
    toggleFavoriteForNote,
    loadSearch,
    loadFavorites,
    loadRecentNotes,
    setSearchSortMode,
    setFavoritesSortMode,
    $reset,
  }
})

if (import.meta.hot) acceptHMRUpdate(useNoteBrowsingStore, import.meta.hot)
