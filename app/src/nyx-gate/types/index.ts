export enum NyxGateDomain {
  Vault = 'vault',
  Note = 'note',
  Feedback = 'feedback',
  User = 'user',
}

export enum MergeOption {
  Overwrite = 'overwrite',
  Merge = 'merge',
}

export interface NyxGateConfig {
  baseUrl?: string
  token?: string | null
  headers?: Record<string, string>
}
