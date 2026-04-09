import type { AuthMode, ServerRole, TeamRole, NotePermission, VaultOwnerType } from './enums'
import type { RouteLocationRaw } from 'vue-router'
import type CommentAnchor from '@/comments/classes/CommentAnchor'

// ─── Shared ───────────────────────────────────────────────────────────────────
export * from './profile'
export * from './enums'
export * from './router'

// ─── Auth ────────────────────────────────────────────────────────────────────

export interface AuthModeResponse {
  mode: AuthMode
  issuer?: string
  client_id?: string
  server_id?: string
  server_name?: string
  api_version?: string
}

export interface ServerMetadata {
  id: string
  slug: string
  name: string
  current_user_id: string
  current_user_username: string
  role: ServerRole
  root_path?: string
}

export interface LoginToken {
  token: string
  expires_in: number
}

export { default as User } from '@/users/classes/User'

export interface ManagedUserSummary {
  id: string
  username: string
  email: string
  display_name: string
  role: ServerRole
  created_at: string
  updated_at: string
  can_edit: boolean
  can_delete: boolean
}

export interface CreateUserRequest {
  username: string
  email: string
  display_name: string
  role: ServerRole
  password: string
}

export interface UpdateUserRequest {
  email: string
  display_name: string
  role: ServerRole
  password?: string
}

// ─── Permissions ─────────────────────────────────────────────────────────────

export { NotePermission, ServerRole, TeamRole, GlobalBrowseSortMode, CommentAttachment, CommentVisibility, AuthMode, WorkspaceProfileType, RemoteConnectionStatus, ProfileSessionState } from './enums'

// ─── Vaults ──────────────────────────────────────────────────────────────────

export { default as Vault } from '@/vaults/classes/Vault'

export type VaultOwner =
  | { type: VaultOwnerType.Home; server_slug: string; home_slug: string }
  | { type: VaultOwnerType.Server; server_slug: string }
  | { type: VaultOwnerType.Local }

// ─── Notes ───────────────────────────────────────────────────────────────────

export { default as NoteMeta } from '@/notes/classes/NoteMeta'
export { default as Note } from '@/notes/classes/Note'
export { default as FeedbackNote } from '@/feedback/classes/FeedbackNote'

export interface NoteOriginContext {
  profile_id: string
  server_label: string
  server_id?: string
  vault_id: string
  vault_name: string
  vault_slug: string
}

export interface FavoriteNoteRef {
  profile_id: string
  vault_id: string
  note_id: string
  created_at: string
}

export interface BrowseNoteCardModel {
  note_id: string
  vault_id: string
  profile_id: string
  title: string
  description?: string
  tags: string[]
  images: string[]
  updated_at: string
  updated_label: string
  href: RouteLocationRaw
  server_label: string
  server_id?: string
  vault_name: string
  vault_slug: string
  is_favorite: boolean
  match_score?: number
}

// ─── Teams ───────────────────────────────────────────────────────────────────

export interface TeamMember {
  user_id: string
  role: TeamRole
}

export interface Team {
  id: string
  name: string
  members: TeamMember[]
}

// ─── Comments ────────────────────────────────────────────────────────────────

export interface CommentReply {
  id: string
  author_id: string
  author_name: string
  body: string
  created_at: string
}

export { default as CommentAnchor } from '@/comments/classes/CommentAnchor'
export { default as Comment } from '@/comments/classes/Comment'

export interface CreateCommentRequest {
  body: string
  anchor: Omit<CommentAnchor, 'attachment' | 'last_matched_at'>
}

// ─── API request shapes ───────────────────────────────────────────────────────

export interface CreateNoteRequest {
  title: string
  content: string
  tags?: string[]
  category?: string
  images?: string[]
  permission?: NotePermission
}

export interface UpdateNoteRequest {
  title: string
  content: string
  tags?: string[]
  category?: string
  images?: string[]
}

export interface FeedbackImageUpload {
  name: string
  mime_type: string
  data: string
}

export interface CreateFeedbackRequest {
  title: string
  description: string
  feedback_type: string
  app_location: string
  storage_path: string
  console_output: string
  interaction_trail?: string | null
  images?: FeedbackImageUpload[]
}

export interface UpdateFeedbackRequest extends CreateFeedbackRequest {}

export interface PatchPermissionRequest {
  permission: NotePermission
}

export interface CreateVaultRequest {
  slug: string
  name: string
  description?: string
  icon?: string
}

export interface UpdateVaultRequest {
  name?: string
  description?: string | null
  icon?: string | null
}

export interface CreateTeamRequest {
  name: string
}

export interface AddMemberRequest {
  user_id: string
  role: Exclude<TeamRole, 'owner'>
}

export interface PatchMemberRequest {
  role: Exclude<TeamRole, 'owner'>
}
