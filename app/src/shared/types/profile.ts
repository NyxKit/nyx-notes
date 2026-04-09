import type { AuthMode, ProfileSessionState, RemoteConnectionStatus, WorkspaceProfileType } from './enums'

export interface WorkspaceProfile {
  id: string
  type: WorkspaceProfileType
  display_name: string
  last_used_at?: string
  last_route?: string
}

export interface LocalWorkspaceProfile extends WorkspaceProfile {
  type: WorkspaceProfileType.Local
}

export interface RemoteWorkspaceProfile extends WorkspaceProfile {
  type: WorkspaceProfileType.Remote
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
  username?: string
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
