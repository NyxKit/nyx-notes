import { LiveSubscriptionStatus, type LiveQuery, type LiveSubscriptionRecord } from '@/shared/types'

type Listener<T> = (record: LiveSubscriptionRecord<T>) => void
type ReconnectHandler = (query: LiveQuery) => void

interface InternalRecord<T> extends LiveSubscriptionRecord<T> {
  listeners: Set<Listener<T>>
  reconnect?: ReconnectHandler
  eventSource?: EventSource
}

function createQueryKey(query: LiveQuery) {
  const normalized = [
    ['collection', query.collection],
    ['scope_kind', query.scope_kind],
    ['server_slug', query.server_slug],
    ['owner_context', query.owner_context ?? ''],
    ['user_context', query.user_context ?? ''],
    ['vault_id', query.vault_id ?? ''],
    ['note_id', query.note_id ?? ''],
    ...((query.filters ?? []).map(([key, value]) => [`filter:${key}`, value] as const)),
  ]

  return normalized.map(([key, value]) => `${key}=${value}`).join('|')
}

class SubscriptionManager {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private records = new Map<string, InternalRecord<any>>()

  connect(query: LiveQuery, reconnect: ReconnectHandler) {
    const key = createQueryKey(query)
    const record = this.records.get(key)
    if (!record) return

    record.reconnect = reconnect
    const eventSource = new EventSource(`/api/live?${this.toSearchParams(query)}`)

    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data)
      if (data.snapshot !== undefined) {
        this.publish(query, data.snapshot)
      }
      if (data.keep_alive === true && record.status === LiveSubscriptionStatus.Reconnecting) {
        record.status = LiveSubscriptionStatus.Active
      }
    }

    eventSource.onerror = () => {
      record.status = LiveSubscriptionStatus.Reconnecting
      record.eventSource?.close()
      setTimeout(() => record.reconnect?.(query), 1000)
    }

    record.eventSource = eventSource
  }

  disconnect(query: LiveQuery) {
    const key = createQueryKey(query)
    const record = this.records.get(key)
    if (!record) return

    record.eventSource?.close()
    record.eventSource = undefined
    record.reconnect = undefined
  }

  private toSearchParams(query: LiveQuery) {
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

  acquire<T = unknown>(query: LiveQuery, listener?: Listener<T>) {
    const key = createQueryKey(query)
    const existing = this.records.get(key) as InternalRecord<T> | undefined

    if (existing) {
      existing.refCount += 1
      existing.status = LiveSubscriptionStatus.Active
      if (listener) existing.listeners.add(listener)
      if (existing.latestSnapshot !== undefined) {
        listener?.(existing)
      }
      return {
        key,
        release: () => this.release(key, listener),
        subscribe: (reconnect: ReconnectHandler) => this.connect(query, reconnect),
        unsubscribe: () => this.disconnect(query),
      }
    }

    const record: InternalRecord<T> = {
      key,
      query,
      status: LiveSubscriptionStatus.Loading,
      refCount: 1,
      generation: 1,
      listeners: new Set(listener ? [listener] : []),
    }

    this.records.set(key, record)
    return {
      key,
      release: () => this.release(key, listener),
      subscribe: (reconnect: ReconnectHandler) => this.connect(query, reconnect),
      unsubscribe: () => this.disconnect(query),
    }
  }

  publish<T = unknown>(query: LiveQuery, snapshot: T) {
    const key = createQueryKey(query)
    const record = this.records.get(key) as InternalRecord<T> | undefined
    if (!record) return

    record.latestSnapshot = snapshot
    record.status = LiveSubscriptionStatus.Active
    record.listeners.forEach(listener => listener(record))
  }

  fail(query: LiveQuery, error: string) {
    const key = createQueryKey(query)
    const record = this.records.get(key)
    if (!record) return

    record.status = LiveSubscriptionStatus.Failed
    record.lastError = error
    record.listeners.forEach(listener => listener(record as never))
  }

  release<T = unknown>(key: string, listener?: Listener<T>) {
    const record = this.records.get(key) as InternalRecord<T> | undefined
    if (!record) return

    if (listener) record.listeners.delete(listener)
    record.refCount = Math.max(0, record.refCount - 1)

    if (record.refCount === 0) {
      record.status = LiveSubscriptionStatus.Released
      record.eventSource?.close()
      this.records.delete(key)
      return
    }

    record.generation += 1
  }

  get<T = unknown>(query: LiveQuery) {
    return this.records.get(createQueryKey(query)) as LiveSubscriptionRecord<T> | undefined
  }

  reset() {
    for (const record of this.records.values()) {
      record.eventSource?.close()
    }
    this.records.clear()
  }
}

export const subscriptionManager = new SubscriptionManager()
export { createQueryKey }
