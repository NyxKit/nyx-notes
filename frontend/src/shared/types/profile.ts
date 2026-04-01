export type AuthMode = 'local' | 'secret_key' | 'oidc'

export type WorkspaceProfileType = 'local' | 'remote'

export type RemoteConnectionStatus =
  | 'unknown'
  | 'reachable'
  | 'unreachable'
  | 'invalid_server'
  | 'unsupported_auth'
  | 'auth_failed'

export type ProfileSessionState =
  | 'signed_out'
  | 'probing'
  | 'signing_in'
  | 'signed_in'
  | 'expired'
  | 'error'

export interface WorkspaceProfile {
  id: string
  type: WorkspaceProfileType
  display_name: string
  last_used_at?: string
  last_route?: string
}

export interface LocalWorkspaceProfile extends WorkspaceProfile {
  type: 'local'
}

export interface RemoteWorkspaceProfile extends WorkspaceProfile {
  type: 'remote'
  server_url: string
  username: string
  server_label?: string
  server_id?: string
  api_version?: string
  connection_status: RemoteConnectionStatus
  auth_mode?: AuthMode
  last_error?: string
}

export type AnyWorkspaceProfile = LocalWorkspaceProfile | RemoteWorkspaceProfile

export interface ProfileSession {
  profile_id: string
  state: ProfileSessionState
  auth_mode?: AuthMode
  token?: string
  expires_at?: string
  last_error?: string
}

export interface StoredWorkspaceProfiles {
  active_profile_id: string | null
  profiles: AnyWorkspaceProfile[]
}

export interface RemoteProfileDraft {
  display_name: string
  server_url: string
  username: string
  password: string
}

export interface ProfileActivationResult {
  profile: AnyWorkspaceProfile
  requires_login: boolean
}
