// ─── Auth ────────────────────────────────────────────────────────────────────

export type AuthMode = 'local' | 'secret_key' | 'oidc'

export interface AuthModeResponse {
  mode: AuthMode
  issuer?: string    // only present when mode === 'oidc'
  client_id?: string // only present when mode === 'oidc'
}

export interface LoginToken {
  token: string
  expires_at: string // ISO 8601
}

export interface User {
  id: string
  email: string
  display_name: string
}

// ─── Permissions ─────────────────────────────────────────────────────────────

export type NotePermission = 'restricted' | 'comment' | 'edit'

export type TeamRole = 'owner' | 'admin' | 'member'

// ─── Vaults ──────────────────────────────────────────────────────────────────

export interface Vault {
  id: string
  slug: string
  name: string
  description?: string
  owner: VaultOwner
  permission: NotePermission
  icon?: string
}

export type VaultOwner =
  | { type: 'user'; id: string }
  | { type: 'team'; id: string }

// ─── Notes ───────────────────────────────────────────────────────────────────

export interface NoteMeta {
  id: string
  vault_id: string
  title: string
  description?: string
  author_id: string
  tags: string[]
  category: string | null
  created_at: string // ISO 8601
  updated_at: string // ISO 8601
  is_encrypted: boolean
  permission: NotePermission
}

export interface Note {
  meta: NoteMeta
  content: string
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

export type CommentAttachment = 'attached' | 'detached'

export type CommentVisibility = 'visible' | 'hidden_legacy'

export interface CommentAnchor {
  text: string
  prefix: string
  suffix: string
  range_from: number
  range_to: number
  attachment: CommentAttachment
  line_preview: string
  last_matched_at?: string
}

export interface Comment {
  id: string
  note_id: string
  author_id: string
  author_name: string
  body: string
  anchor: CommentAnchor
  resolved: boolean
  visibility: CommentVisibility
  created_at: string
  updated_at: string
  replies: CommentReply[]
}

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
  permission?: NotePermission
}

export interface UpdateNoteRequest {
  title: string
  content: string
  tags?: string[]
  category?: string
}

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
