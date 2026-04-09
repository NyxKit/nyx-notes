export enum LiveCollection {
  VaultListPersonal = 'vault_list_personal',
  VaultListShared = 'vault_list_shared',
  NoteList = 'note_list',
  Note = 'note',
}

export enum LiveScopeKind {
  Collection = 'collection',
  Document = 'document',
}

export enum LiveSubscriptionStatus {
  Loading = 'loading',
  Active = 'active',
  Reconnecting = 'reconnecting',
  Failed = 'failed',
  Released = 'released',
}

export interface LiveQuery {
  collection: LiveCollection
  scope_kind: LiveScopeKind
  server_slug: string
  owner_context?: string
  user_context?: string
  vault_id?: string
  note_id?: string
  filters?: Array<[string, string]>
}

export interface CanonicalQueryKey {
  resource_name: string
  scope_kind: LiveScopeKind
  normalized_identifiers: Array<[string, string]>
  normalized_filters: Array<[string, string]>
}

export interface SharedSubscriptionSnapshotMap {
  [LiveCollection.VaultListPersonal]: unknown
  [LiveCollection.VaultListShared]: unknown
  [LiveCollection.NoteList]: unknown
  [LiveCollection.Note]: unknown
}

export interface LiveSubscriptionRecord<T> {
  key: string
  query: LiveQuery
  status: LiveSubscriptionStatus
  refCount: number
  generation: number
  latestSnapshot?: T
  lastError?: string
}
