import { describe, expect, it, vi, beforeEach } from 'vitest'
import { subscriptionManager } from './subscriptionManager'
import { LiveCollection, LiveScopeKind } from '@/shared/types'

vi.stubGlobal('EventSource', vi.fn().mockImplementation(() => ({
  onmessage: null,
  onerror: null,
  close: vi.fn(),
})))

describe('subscriptionManager generation fencing', () => {
  beforeEach(() => {
    subscriptionManager.reset()
  })

  it('increments generation on release when refCount > 0', () => {
    const query = {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    }

    subscriptionManager.acquire(query)
    const record1 = subscriptionManager.get(query)
    const gen1 = record1!.generation

    subscriptionManager.acquire(query).release()
    const recordAfterRelease = subscriptionManager.get(query)
    expect(recordAfterRelease!.generation).toBe(gen1 + 1)
  })

  it('rejects updates after release by checking generation', () => {
    const query = {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    }

    subscriptionManager.acquire(query)
    const genAtRelease = subscriptionManager.get(query)!.generation

    subscriptionManager.acquire(query).release()

    const listener = vi.fn()
    subscriptionManager.acquire(query, listener)

    subscriptionManager.publish(query, [{ id: 'note-new' }])

    const lastCallGen = listener.mock.calls[listener.mock.calls.length - 1][0].generation
    expect(lastCallGen).toBeGreaterThan(genAtRelease)
  })

  it('allows updates within the same generation', () => {
    const query = {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    }

    const listener = vi.fn()
    subscriptionManager.acquire(query, listener)

    const gen1 = subscriptionManager.get(query)!.generation

    subscriptionManager.publish(query, [{ id: 'note-1' }])
    expect(listener).toHaveBeenCalledTimes(1)
    expect(listener.mock.calls[0][0].generation).toBe(gen1)

    subscriptionManager.publish(query, [{ id: 'note-2' }])
    expect(listener).toHaveBeenCalledTimes(2)
    expect(listener.mock.calls[1][0].generation).toBe(gen1)
  })

  it('new subscriber gets current generation', () => {
    const query = {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    }

    subscriptionManager.acquire(query)
    subscriptionManager.publish(query, [{ id: 'note-1' }])

    const listener = vi.fn()
    subscriptionManager.acquire(query, listener)

    const record = subscriptionManager.get(query)
    expect(listener).toHaveBeenCalledWith(
      expect.objectContaining({ generation: record!.generation })
    )
  })
})