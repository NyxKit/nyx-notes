import { beforeEach, describe, expect, it } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

describe('useNoteBrowsingStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorage.clear()
  })

  it('records search queries without loading data', async () => {
    const { useNoteBrowsingStore } = await import('./noteBrowsing')
    const store = useNoteBrowsingStore()

    await store.loadSearch('alpha')
    expect(store.searchQuery).toBe('alpha')
    expect(store.searchLoading).toBe(false)
    expect(store.searchResults).toEqual([])
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
