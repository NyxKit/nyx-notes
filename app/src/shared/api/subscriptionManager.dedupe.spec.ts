import { describe, expect, it, vi, beforeEach } from 'vitest'
import { subscriptionManager, createQueryKey } from './subscriptionManager'
import { LiveCollection, LiveScopeKind } from '@/shared/types'

vi.stubGlobal('EventSource', vi.fn().mockImplementation(() => ({
  onmessage: null,
  onerror: null,
  close: vi.fn(),
})))

describe('subscriptionManager', () => {
  beforeEach(() => {
    subscriptionManager.reset()
  })

  describe('ref-counting and deduplication', () => {
    it('increments refCount when acquiring the same query twice', () => {
    const query = {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: 'main-server',
      vault_id: 'writing',
    }

    subscriptionManager.acquire(query)
    const record1 = subscriptionManager.get(query)
    expect(record1?.refCount).toBe(1)

    subscriptionManager.acquire(query)
    const record2 = subscriptionManager.get(query)
    expect(record2?.refCount).toBe(2)

    subscriptionManager.acquire(query).release()
    const recordAfterRelease = subscriptionManager.get(query)
    expect(recordAfterRelease?.refCount).toBe(1)

    subscriptionManager.acquire(query).release()
    expect(subscriptionManager.get(query)).toBeUndefined()
  })

    it('deduplicates multiple listeners for the same query', () => {
      const query = {
        collection: LiveCollection.VaultListPersonal,
        scope_kind: LiveScopeKind.Collection,
        server_slug: 'main-server',
      }

      const listener1 = vi.fn()
      const listener2 = vi.fn()

      subscriptionManager.acquire(query, listener1)
      subscriptionManager.acquire(query, listener2)

      subscriptionManager.publish(query, [{ slug: 'test-vault', name: 'Test' }])

      expect(listener1).toHaveBeenCalledTimes(1)
      expect(listener2).toHaveBeenCalledTimes(1)
    })

    it('replays latest snapshot to new listeners', () => {
      const query = {
        collection: LiveCollection.NoteList,
        scope_kind: LiveScopeKind.Collection,
        server_slug: 'main-server',
        vault_id: 'writing',
      }

      // First acquire creates the record
      subscriptionManager.acquire(query)

      // Publish sets the snapshot
      subscriptionManager.publish(query, [{ id: 'note-1', title: 'First' }])

      // New listener gets the latest snapshot
      const listener = vi.fn()
      subscriptionManager.acquire(query, listener)

      expect(listener).toHaveBeenCalledWith(
        expect.objectContaining({
          latestSnapshot: [{ id: 'note-1', title: 'First' }],
        })
      )
    })

    it('uses canonical query keys for deduplication', () => {
      const query1 = {
        collection: LiveCollection.NoteList,
        scope_kind: LiveScopeKind.Collection,
        server_slug: 'main-server',
        vault_id: 'writing',
        filters: [['tag', 'draft']] as [string, string][],
      }

      const query2 = {
        collection: LiveCollection.NoteList,
        scope_kind: LiveScopeKind.Collection,
        server_slug: 'main-server',
        vault_id: 'writing',
        filters: [['tag', 'draft']] as [string, string][],
      }

      const key1 = createQueryKey(query1)
      const key2 = createQueryKey(query2)

      expect(key1).toBe(key2)
    })

    it('distinguishes queries with different filters', () => {
      const query1 = {
        collection: LiveCollection.NoteList,
        scope_kind: LiveScopeKind.Collection,
        server_slug: 'main-server',
        vault_id: 'writing',
        filters: [['tag', 'draft']] as [string, string][],
      }

      const query2 = {
        collection: LiveCollection.NoteList,
        scope_kind: LiveScopeKind.Collection,
        server_slug: 'main-server',
        vault_id: 'writing',
        filters: [['tag', 'published']] as [string, string][],
      }

      const key1 = createQueryKey(query1)
      const key2 = createQueryKey(query2)

      expect(key1).not.toBe(key2)
    })
  })
})