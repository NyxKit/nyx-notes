import { beforeEach, describe, expect, it, vi } from 'vitest'
import { LiveCollection, LiveScopeKind } from '@/shared/types'

const apiMock = vi.fn().mockResolvedValue({ status: 'not_implemented' })
const fetchVaultsMock = vi.fn().mockResolvedValue([{ slug: 'writing', name: 'Writing' }])
const fetchNotesMock = vi.fn().mockResolvedValue([{ id: 'note-1', vault_id: 'writing', title: 'Draft' }])
const fetchNoteMock = vi.fn().mockResolvedValue({ meta: { id: 'note-1', vault_id: 'writing', title: 'Draft' }, content: 'Hello' })

vi.mock('@/shared/api/client', () => ({
  api: apiMock,
}))

vi.mock('@/vaults/api', () => ({
  fetchVaults: fetchVaultsMock,
}))

vi.mock('@/notes/api', () => ({
  fetchNotes: fetchNotesMock,
  fetchNote: fetchNoteMock,
}))

vi.stubGlobal('EventSource', vi.fn().mockImplementation(() => ({
  onmessage: null,
  onerror: null,
  close: vi.fn(),
})))

describe('NyxBase', () => {
  beforeEach(async () => {
    vi.clearAllMocks()
    const { subscriptionManager } = await import('./subscriptionManager')
    subscriptionManager.reset()
  })

  it('creates note list queries with the expected scope', async () => {
    const { NyxBase } = await import('./vaultBase')

    expect(NyxBase.createNoteListQuery('main-server', 'writing')).toEqual({
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    })
  })

  it('loads an initial note list snapshot before live updates', async () => {
    const { NyxBase } = await import('./vaultBase')

    const query = NyxBase.createNoteListQuery('main-server', 'writing')
    const handle = NyxBase.subscribe(query, vi.fn())

    await vi.waitFor(() => {
      expect(fetchNotesMock).toHaveBeenCalledWith('writing')
      expect(handle.key).toBe('vault_writing')
    })
  })
})
