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

export const VaultBase = {
  subscribe<T = unknown>(query: LiveQuery, onUpdate?: (snapshot: T) => void) {
    const handle = subscriptionManager.acquire<T>(query, record => {
      if (record.latestSnapshot !== undefined) {
        onUpdate?.(record.latestSnapshot)
      }
    })

    void Promise.resolve().then(async () => {
      switch (query.collection) {
        case LiveCollection.VaultListPersonal:
        case LiveCollection.VaultListShared:
          subscriptionManager.publish(query, await fetchVaults() as T)
          break
        case LiveCollection.NoteList:
          if (!query.vault_id) throw new Error('vault_id is required')
          subscriptionManager.publish(query, await fetchNotes(query.vault_id) as T)
          break
        case LiveCollection.Note:
          if (!query.vault_id || !query.note_id) throw new Error('vault_id and note_id are required')
          subscriptionManager.publish(query, await fetchNote(query.vault_id, query.note_id) as T)
          break
      }

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const response: any = await api(`/api/live?${toSearchParams(query).toString()}`)
      if (response?.data !== undefined) {
        subscriptionManager.publish(query, response.data as T)
      }
    }).catch((error: unknown) => {
      subscriptionManager.fail(query, error instanceof Error ? error.message : String(error))
    })

    const reconnect = async (q: LiveQuery) => {
      try {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const response: any = await api(`/api/live?${toSearchParams(q).toString()}`)
        if (response?.data !== undefined) {
          subscriptionManager.publish(q, response.data as T)
        }
      } catch (error: unknown) {
        subscriptionManager.fail(q, error instanceof Error ? error.message : String(error))
      }
    }

    handle.subscribe(reconnect)

    return {
      key: handle.key,
      release: () => {
        handle.unsubscribe()
        handle.release()
      },
    }
  },

  createVaultListPersonalQuery(serverSlug: string, userContext?: string): LiveQuery {
    return {
      collection: LiveCollection.VaultListPersonal,
      scope_kind: LiveScopeKind.Collection,
      server_slug: serverSlug,
      user_context: userContext,
    }
  },

  createVaultListSharedQuery(serverSlug: string): LiveQuery {
    return {
      collection: LiveCollection.VaultListShared,
      scope_kind: LiveScopeKind.Collection,
      server_slug: serverSlug,
    }
  },

  createNoteListQuery(serverSlug: string, vaultId: string): LiveQuery {
    return {
      collection: LiveCollection.NoteList,
      scope_kind: LiveScopeKind.Collection,
      server_slug: serverSlug,
      vault_id: vaultId,
    }
  },

  createNoteQuery(serverSlug: string, vaultId: string, noteId: string): LiveQuery {
    return {
      collection: LiveCollection.Note,
      scope_kind: LiveScopeKind.Document,
      server_slug: serverSlug,
      vault_id: vaultId,
      note_id: noteId,
    }
  },
}
