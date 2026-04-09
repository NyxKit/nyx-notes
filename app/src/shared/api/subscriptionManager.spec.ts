import { beforeEach, describe, expect, it } from 'vitest'
import { LiveCollection, LiveScopeKind } from '@/shared/types'
import { createQueryKey, subscriptionManager } from './subscriptionManager'

const query = {
  collection: LiveCollection.NoteList,
  scope_kind: LiveScopeKind.Collection,
  server_slug: 'main-server',
  vault_id: 'writing',
}

describe('subscriptionManager', () => {
  beforeEach(() => {
    subscriptionManager.reset()
  })

  it('replays the latest snapshot to later listeners of the same query', () => {
    const updates: string[][] = []

    subscriptionManager.acquire<string[]>(query, record => {
      if (record.latestSnapshot) updates.push(record.latestSnapshot)
    })
    subscriptionManager.publish(query, ['note-1'])

    subscriptionManager.acquire<string[]>(query, record => {
      if (record.latestSnapshot) updates.push(record.latestSnapshot)
    })

    expect(updates).toEqual([['note-1'], ['note-1']])
    expect(subscriptionManager.get<string[]>(query)?.refCount).toBe(2)
  })

  it('uses a stable canonical key for equivalent queries', () => {
    expect(createQueryKey(query)).toBe(
      createQueryKey({
        vault_id: 'writing',
        server_slug: 'main-server',
        scope_kind: LiveScopeKind.Collection,
        collection: LiveCollection.NoteList,
      }),
    )
  })
})
