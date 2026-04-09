export enum AuthMode {
  Local = 'local',
  SecretKey = 'secret_key',
  Oidc = 'oidc',
}

export enum WorkspaceProfileType {
  Local = 'local',
  Remote = 'remote',
}

export enum VaultOwnerType {
  Home = 'home',
  Server = 'server',
  Local = 'local',
}

export enum RemoteConnectionStatus {
  Unknown = 'unknown',
  Reachable = 'reachable',
  Unreachable = 'unreachable',
  InvalidServer = 'invalid_server',
  UnsupportedAuth = 'unsupported_auth',
  AuthFailed = 'auth_failed',
}

export enum ProfileSessionState {
  SignedOut = 'signed_out',
  Probing = 'probing',
  SigningIn = 'signing_in',
  SignedIn = 'signed_in',
  Expired = 'expired',
  Error = 'error',
}

export enum NotePermission {
  Restricted = 'restricted',
  Comment = 'comment',
  Edit = 'edit',
}

export enum ServerRole {
  Admin = 'admin',
  User = 'user',
}

export enum TeamRole {
  Owner = 'owner',
  Admin = 'admin',
  Member = 'member',
}

export enum GlobalBrowseSortMode {
  BestMatch = 'best_match',
  Recent = 'recent',
  Grouped = 'grouped',
}

export enum CommentAttachment {
  Attached = 'attached',
  Detached = 'detached',
}

export enum CommentVisibility {
  Visible = 'visible',
  HiddenLegacy = 'hidden_legacy',
}
