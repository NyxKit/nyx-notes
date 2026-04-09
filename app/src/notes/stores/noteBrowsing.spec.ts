import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { GlobalBrowseSortMode } from '@/shared/types'

const loadSearchResults = vi.fn()
const loadFavoriteResults = vi.fn()

vi.mock('@/notes/composables', () => ({
  useGlobalNoteBrowsing: () => ({
    loadSearchResults,
    loadFavoriteResults,
  }),
}))

describe('useNoteBrowsingStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorage.clear()
    vi.clearAllMocks()
  })

  it('sorts search results by best match and grouped order', async () => {
    const { useNoteBrowsingStore } = await import('./noteBrowsing')
    const store = useNoteBrowsingStore()

    loadSearchResults.mockResolvedValue({
      excluded_profiles_count: 0,
      results: [
        {
          note_id: 'a',
          vault_id: 'vault-2',
          profile_id: 'profile-2',
          title: 'A',
          tags: [],
          updated_at: '2026-04-01T10:00:00Z',
          updated_label: '2h ago',
          href: '/vaults/vault-2/notes/a?profile=profile-2',
          server_label: 'Beta',
          vault_name: 'Zeta',
          vault_slug: 'zeta',
          is_favorite: false,
          match_score: 20,
        },
        {
          note_id: 'b',
          vault_id: 'vault-1',
          profile_id: 'profile-1',
          title: 'B',
          tags: [],
          updated_at: '2026-04-01T11:00:00Z',
          updated_label: '1h ago',
          href: '/vaults/vault-1/notes/b?profile=profile-1',
          server_label: 'Alpha',
          vault_name: 'Alpha Vault',
          vault_slug: 'alpha-vault',
          is_favorite: false,
          match_score: 80,
        },
      ],
    })

    await store.loadSearch('alpha')
    expect(store.searchResults.map(note => note.note_id)).toEqual(['b', 'a'])

     store.setSearchSortMode(GlobalBrowseSortMode.Grouped)
    expect(store.searchResults.map(note => note.note_id)).toEqual(['b', 'a'])
  })

  it('toggles favorites and persists compound refs', async () => {
    const { useNoteBrowsingStore } = await import('./noteBrowsing')
    const store = useNoteBrowsingStore()

    store.toggleFavorite('profile-1', 'vault-1', 'note-1')

    expect(store.isFavorite('profile-1', 'vault-1', 'note-1')).toBe(true)
    expect(JSON.parse(localStorage.getItem('nyx_favorites_v2') ?? '[]')).toEqual([
      expect.objectContaining({
        profile_id: 'profile-1',
        vault_id: 'vault-1',
        note_id: 'note-1',
      }),
    ])
  })
})
