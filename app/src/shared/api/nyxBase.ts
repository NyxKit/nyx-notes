import { getApiBaseUrl, getApiToken } from '@/shared/api/client'
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
  switch (query.collection) {
    case LiveCollection.VaultListPersonal:
    case LiveCollection.VaultListShared:
      return `vaults_${query.collection}_${query.server_slug}_${query.user_context ?? 'all'}`
    case LiveCollection.NoteList:
      return `vault_${query.vault_id}_notes`
    case LiveCollection.Note:
      return `vault_${query.vault_id}_note_${query.note_id}`
  }

  return `server_${query.server_slug}`
}

interface SubscriptionEntry {
  key: string
  query: LiveQuery
  refCount: number
  onUpdate: (snapshot: unknown) => void
  abortController?: AbortController
  reconnectAttempts: number
}

function parseSseFrame(frame: string) {
  let event = 'message'
  const dataLines: string[] = []

  for (const line of frame.split('\n')) {
    if (line.startsWith('event:')) {
      event = line.slice(6).trim()
    } else if (line.startsWith('data:')) {
      dataLines.push(line.slice(5).trim())
    }
  }

  return { event, data: dataLines.join('\n') }
}

function liveUrl(query: LiveQuery) {
  const path = `/api/live?${toSearchParams(query).toString()}`
  const base = getApiBaseUrl()
  if (!base || base === '/') return path
  return new URL(path, base).toString()
}

function liveHeaders() {
  const headers = new Headers()
  const token = getApiToken()
  if (token) headers.set('Authorization', `Bearer ${token}`)
  return headers
}

class NyxBaseClass {
  private subscriptions = new Map<string, SubscriptionEntry>()
  private maxReconnectAttempts = 5
  private baseRetryDelay = 1000

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
      reconnectAttempts: 0,
    }
    this.subscriptions.set(vaultKey, entry)

    this.loadInitialSnapshot(query, vaultKey, onUpdate)

    return {
      key: vaultKey,
      release: () => this.release(vaultKey),
    }
  }

  private async loadInitialSnapshot<T>(query: LiveQuery, vaultKey: string, onUpdate: (snapshot: T) => void) {
    const maxRetries = 3
    let lastError: string = 'Unknown error'

    for (let attempt = 0; attempt < maxRetries; attempt++) {
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

        if (this.subscriptions.has(vaultKey)) {
          this.startLiveConnection(query, vaultKey)
        }
        return
      } catch (error) {
        lastError = error instanceof Error ? error.message : String(error)

        if (error instanceof TypeError && error.message.includes('fetch')) {
          if (attempt < maxRetries - 1) {
            await new Promise(resolve => setTimeout(resolve, 1000 * (attempt + 1)))
            continue
          }
        }

        break
      }
    }

    subscriptionManager.fail(query, `Failed to load snapshot after ${maxRetries} attempts: ${lastError}`)
  }

  private startLiveConnection(query: LiveQuery, vaultKey: string) {
    const entry = this.subscriptions.get(vaultKey)
    if (!entry) return
    if (entry.abortController) return
    if (entry.reconnectAttempts >= this.maxReconnectAttempts) {
      subscriptionManager.fail(query, 'Max reconnection attempts reached')
      return
    }

    const controller = new AbortController()
    entry.abortController = controller

    void (async () => {
      try {
        const response = await fetch(liveUrl(query), {
          method: 'GET',
          headers: liveHeaders(),
          signal: controller.signal,
        })

        if (!response.ok || !response.body) {
          throw new Error(`Live request failed with ${response.status}`)
        }

        const reader = response.body.getReader()
        const decoder = new TextDecoder()
        let buffer = ''

        while (true) {
          const { done, value } = await reader.read()
          if (done) break

          buffer += decoder.decode(value, { stream: true })

          while (buffer.includes('\n\n')) {
            const index = buffer.indexOf('\n\n')
            const frame = buffer.slice(0, index)
            buffer = buffer.slice(index + 2)
            const { data } = parseSseFrame(frame)
            if (!data) continue

            try {
              const payload = JSON.parse(data)
              if (payload.type === 'snapshot' && payload.data !== undefined) {
                subscriptionManager.publish(query, payload.data)
                entry.onUpdate(payload.data)
                entry.reconnectAttempts = 0
              }
            } catch (parseError) {
              console.error('Failed to parse live payload:', parseError)
            }
          }
        }

        if (!controller.signal.aborted && this.subscriptions.has(vaultKey)) {
          entry.abortController = undefined
          entry.reconnectAttempts += 1
          const delay = this.baseRetryDelay * Math.pow(2, entry.reconnectAttempts - 1)
          const cappedDelay = Math.min(delay, 30000)

          if (entry.reconnectAttempts < this.maxReconnectAttempts) {
            setTimeout(() => {
              if (this.subscriptions.has(vaultKey)) this.startLiveConnection(query, vaultKey)
            }, cappedDelay)
          } else {
            subscriptionManager.fail(query, `Failed to connect after ${entry.reconnectAttempts} attempts`)
          }
        }
      } catch (error) {
        if (controller.signal.aborted) return

        entry.abortController = undefined
        entry.reconnectAttempts += 1

        const delay = this.baseRetryDelay * Math.pow(2, entry.reconnectAttempts - 1)
        const cappedDelay = Math.min(delay, 30000)

        if (entry.reconnectAttempts < this.maxReconnectAttempts && this.subscriptions.has(vaultKey)) {
          setTimeout(() => {
            if (this.subscriptions.has(vaultKey)) this.startLiveConnection(query, vaultKey)
          }, cappedDelay)
        } else {
          const message = error instanceof Error ? error.message : String(error)
          subscriptionManager.fail(query, `Live stream error: ${message}`)
        }
      }
    })()
  }

  private release(vaultKey: string) {
    const entry = this.subscriptions.get(vaultKey)
    if (!entry) return

    entry.refCount = Math.max(0, entry.refCount - 1)

    if (entry.refCount === 0) {
      entry.abortController?.abort()
      this.subscriptions.delete(vaultKey)
    }
  }

  reset() {
    for (const entry of this.subscriptions.values()) {
      entry.abortController?.abort()
    }
    this.subscriptions.clear()
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

export const NyxBase = new NyxBaseClass()
