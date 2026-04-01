import { mount } from '@vue/test-utils'
import { ref } from 'vue'
import { describe, expect, it, vi } from 'vitest'
import FavoritesView from './FavoritesView.vue'

const loadFavorites = vi.fn()
const favoriteResults = ref([])
const favoriteRefs = ref([])
const favoritesSortMode = ref<'recent' | 'grouped'>('recent')
const favoritesLoading = ref(false)
const favoritesError = ref<string | null>(null)
const favoritesExcludedProfilesCount = ref(0)

vi.mock('@/notes/stores', () => ({
  useNoteBrowsingStore: () => ({
    favoriteResults,
    favoriteRefs,
    favoritesSortMode,
    favoritesLoading,
    favoritesError,
    favoritesExcludedProfilesCount,
    loadFavorites,
    setFavoritesSortMode: vi.fn(),
  }),
}))

describe('FavoritesView', () => {
  it('passes favorites-only sort options to the shared browse surface', async () => {
    const wrapper = mount(FavoritesView, {
      global: {
        stubs: {
          GlobalNoteBrowseView: {
            props: ['sortOptions', 'title'],
            template: '<div>{{ title }}|{{ sortOptions.map(option => option.value).join(",") }}</div>',
          },
        },
      },
    })

    expect(loadFavorites).toHaveBeenCalled()
    expect(wrapper.text()).toContain('Favorites|recent,grouped')
  })
})
