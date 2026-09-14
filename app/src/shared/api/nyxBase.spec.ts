import { beforeEach, describe, expect, it, vi } from 'vitest'
import { LiveCollection, LiveScopeKind } from '@/shared/types'
import { setApiToken } from './client'

const fetchVaultsMock = vi.fn().mockResolvedValue([{ slug: 'writing', name: 'Writing' }])
const fetchNotesMock = vi.fn().mockResolvedValue([{ id: 'note-1', vault_id: 'writing', title: 'Draft' }])
const fetchNoteMock = vi.fn().mockResolvedValue({ meta: { id: 'note-1', vault_id: 'writing', title: 'Draft' }, content: 'Hello' })

vi.mock('@/vaults/api', () => ({
  fetchVaults: fetchVaultsMock,
}))

vi.mock('@/notes/api', () => ({
  fetchNotes: fetchNotesMock,
  fetchNote: fetchNoteMock,
}))

const fetchMock = vi.fn()

describe('NyxBase', () => {
  beforeEach(async () => {
    vi.clearAllMocks()
    setApiToken(null)
    vi.stubGlobal('fetch', fetchMock)
    const { subscriptionManager } = await import('./subscriptionManager')
    const { NyxBase } = await import('./nyxBase')
    subscriptionManager.reset()
    NyxBase.reset()
  })

  it('creates note list queries with the expected scope', async () => {
    const { NyxBase } = await import('./nyxBase')

    expect(NyxBase.createNoteListQuery('main-server', 'writing')).toEqual({
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    })
  })

  it('loads an initial note list snapshot before live updates', async () => {
    const { NyxBase } = await import('./nyxBase')

    const query = NyxBase.createNoteListQuery('main-server', 'writing')
    const handle = NyxBase.subscribe(query, vi.fn())

    await vi.waitFor(() => {
      expect(fetchNotesMock).toHaveBeenCalledWith('writing')
      expect(handle.key).toBe('vault_writing_notes')
    })
  })

  it('uses distinct keys for note lists and note documents', async () => {
    const { NyxBase } = await import('./nyxBase')

    const listHandle = NyxBase.subscribe(NyxBase.createNoteListQuery('main-server', 'writing'), vi.fn())
    const noteHandle = NyxBase.subscribe(NyxBase.createNoteQuery('main-server', 'writing', 'note-1'), vi.fn())

    expect(listHandle.key).toBe('vault_writing_notes')
    expect(noteHandle.key).toBe('vault_writing_note_note-1')
  })

  it('sends the bearer token on live stream requests', async () => {
    fetchMock.mockResolvedValueOnce({
      ok: true,
      body: {
        getReader: () => ({
          read: vi.fn().mockResolvedValue({ done: true, value: undefined }),
        }),
      },
    })

    setApiToken('secret-token')

    const { NyxBase } = await import('./nyxBase')
    NyxBase.subscribe(NyxBase.createNoteListQuery('main-server', 'writing'), vi.fn())

    await vi.waitFor(() => {
      expect(fetchMock).toHaveBeenCalled()
      const [, init] = fetchMock.mock.calls[0]
      const headers = new Headers(init?.headers as HeadersInit | undefined)
      expect(headers.get('Authorization')).toBe('Bearer secret-token')
    })
  })
})
