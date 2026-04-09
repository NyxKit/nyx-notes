import { api } from '@/shared/api/client'
import { subscriptionManager } from './subscriptionManager'
import { LiveCollection, LiveScopeKind, type LiveQuery } from '@/shared/types'
import { fetchVaults } from '@/vaults/api'
import { fetchNote, fetchNotes } from '@/notes/api'

function toSearchParams(query: LiveQuery) {
  const params = new URLSearchParams({
    collection: query.collection,
    scope_kind: query.scope_kind,
    server_slug: query.server_slug,
  })

  if (query.owner_context) params.set('owner_context', query.owner_context)
  if (query.user_context) params.set('user_context', query.user_context)
  if (query.vault_id) params.set('vault_id', query.vault_id)
  if (query.note_id) params.set('note_id', query.note_id)

  return params
}

function createVaultKey(query: LiveQuery): string {
  if (query.vault_id) {
    return `vault_${query.vault_id}`
  }
  return `server_${query.server_slug}`
}

interface SubscriptionEntry {
  key: string
  query: LiveQuery
  refCount: number
  onUpdate: (snapshot: unknown) => void
  eventSource?: EventSource
}

class VaultBaseClass {
  private subscriptions = new Map<string, SubscriptionEntry>()

  subscribe<T>(query: LiveQuery, onUpdate: (snapshot: T) => void): { key: string; release: () => void } {
    const vaultKey = createVaultKey(query)
    const existing = this.subscriptions.get(vaultKey)

    if (existing) {
      existing.refCount += 1
      existing.onUpdate = onUpdate as (snapshot: unknown) => void
      if (existing.refCount === 2) {
        this.startLiveConnection(query, vaultKey)
      }
      return {
        key: vaultKey,
        release: () => this.release(vaultKey),
      }
    }

    const entry: SubscriptionEntry = {
      key: vaultKey,
      query,
      refCount: 1,
      onUpdate: onUpdate as (snapshot: unknown) => void,
    }
    this.subscriptions.set(vaultKey, entry)

    this.loadInitialSnapshot(query, vaultKey, onUpdate)

    return {
      key: vaultKey,
      release: () => this.release(vaultKey),
    }
  }

  private async loadInitialSnapshot<T>(query: LiveQuery, vaultKey: string, onUpdate: (snapshot: T) => void) {
    try {
      let snapshot: unknown
      switch (query.collection) {
        case LiveCollection.VaultListPersonal:
        case LiveCollection.VaultListShared:
          snapshot = await fetchVaults()
          break
        case LiveCollection.NoteList:
          if (!query.vault_id) throw new Error('vault_id is required')
          snapshot = await fetchNotes(query.vault_id)
          break
        case LiveCollection.Note:
          if (!query.vault_id || !query.note_id) throw new Error('vault_id and note_id are required')
          snapshot = await fetchNote(query.vault_id, query.note_id)
          break
      }

      if (snapshot !== undefined) {
        subscriptionManager.publish(query, snapshot)
        onUpdate(snapshot as T)
      }

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const response: any = await api(`/api/live?${toSearchParams(query).toString()}`)
      if (response?.data !== undefined) {
        subscriptionManager.publish(query, response.data)
        onUpdate(response.data as T)
      }

      if (this.subscriptions.has(vaultKey)) {
        this.startLiveConnection(query, vaultKey)
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error)
      subscriptionManager.fail(query, message)
    }
  }

  private startLiveConnection(query: LiveQuery, vaultKey: string) {
    const entry = this.subscriptions.get(vaultKey)
    if (!entry) return
    if (entry.eventSource) return

    const eventSource = new EventSource(`/api/live?${toSearchParams(query).toString()}`)

    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data)
      if (data.snapshot !== undefined) {
        subscriptionManager.publish(query, data.snapshot)
        entry.onUpdate(data.snapshot)
      }
    }

    eventSource.onerror = () => {
      eventSource.close()
      entry.eventSource = undefined
      setTimeout(() => {
        if (this.subscriptions.has(vaultKey)) {
          this.startLiveConnection(query, vaultKey)
        }
      }, 1000)
    }

    entry.eventSource = eventSource
  }

  private release(vaultKey: string) {
    const entry = this.subscriptions.get(vaultKey)
    if (!entry) return

    entry.refCount = Math.max(0, entry.refCount - 1)

    if (entry.refCount === 0) {
      entry.eventSource?.close()
      this.subscriptions.delete(vaultKey)
    }
  }

  createVaultListPersonalQuery(serverSlug: string, userContext?: string): LiveQuery {
    return {
      collection: LiveCollection.VaultListPersonal,
      scope_kind: LiveScopeKind.Collection,
      server_slug: serverSlug,
      user_context: userContext,
    }
  }

  createVaultListSharedQuery(serverSlug: string): LiveQuery {
    return {
      collection: LiveCollection.VaultListShared,
      scope_kind: LiveScopeKind.Collection,
      server_slug: serverSlug,
    }
  }

  createNoteListQuery(serverSlug: string, vaultId: string): LiveQuery {
    return {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: serverSlug,
      vault_id: vaultId,
    }
  }

  createNoteQuery(serverSlug: string, vaultId: string, noteId: string): LiveQuery {
    return {
      collection: LiveCollection.Note,
      scope_kind: LiveScopeKind.Document,
      server_slug: serverSlug,
      vault_id: vaultId,
      note_id: noteId,
    }
  }
}

export const VaultBase = new VaultBaseClass()
