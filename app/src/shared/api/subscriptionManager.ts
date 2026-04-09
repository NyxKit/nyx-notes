import { LiveSubscriptionStatus, type LiveQuery, type LiveSubscriptionRecord } from '@/shared/types'

type Listener<T> = (record: LiveSubscriptionRecord<T>) => void

interface InternalRecord<T> extends LiveSubscriptionRecord<T> {
  listeners: Set<Listener<T>>
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
  private records = new Map<string, InternalRecord<any>>()

  acquire<T = unknown>(query: LiveQuery, listener?: Listener<T>) {
    const key = createQueryKey(query)
    const existing = this.records.get(key) as InternalRecord<T> | undefined

    if (existing) {
      existing.refCount += 1
      existing.status = LiveSubscriptionStatus.Active
      if (listener) existing.listeners.add(listener)
      listener?.(existing)
      return { key, release: () => this.release(key, listener) }
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
    listener?.(record)
    return { key, release: () => this.release(key, listener) }
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
      this.records.delete(key)
      return
    }

    record.generation += 1
  }

  get<T = unknown>(query: LiveQuery) {
    return this.records.get(createQueryKey(query)) as LiveSubscriptionRecord<T> | undefined
  }

  reset() {
    this.records.clear()
  }
}

export const subscriptionManager = new SubscriptionManager()
export { createQueryKey }
