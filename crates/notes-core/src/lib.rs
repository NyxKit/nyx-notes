pub mod auth;
pub mod domain;
pub mod live;
pub mod storage;

#[cfg(test)]
mod tests;

pub use auth::{
    AuthError, AuthStore, CreateUserInput, LoginToken, ManagedUserSummary, UpdateUserInput, User,
};
pub use domain::{
    distill_markdown_description, slugify, Comment, CommentAnchor, CommentAttachment, CommentReply,
    CommentVisibility, Note, NoteMeta, NotePermission, ServerRole, Vault, VaultIconUpdate,
    VaultOwner, VaultUpdate,
};
pub use live::{
    CanonicalQueryKey, LiveCollection, LiveQuery, LiveScopeKind, LiveSubscriptionStatus,
};
pub use storage::{StorageBackend, StorageError, SyncResult};
