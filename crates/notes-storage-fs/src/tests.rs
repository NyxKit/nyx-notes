use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Utc;
use notes_core::{
    Comment, CommentAnchor, CommentAttachment, CommentReply, CommentVisibility, NotePermission,
    StorageBackend, Vault, VaultOwner, VaultUpdate,
};

use crate::FsStorage;

fn temp_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("nyx-notes-fs-test-{nanos}"))
}

#[test]
fn update_vault_persists_description_to_vault_json() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let vault = Vault {
        id: "vault-1".into(),
        slug: "writing".into(),
        name: "Writing".into(),
        description: None,
        owner: VaultOwner::User("user-1".into()),
        permission: NotePermission::Restricted,
        icon: Some("folder".into()),
    };

    storage.create_vault(&vault).unwrap();
    storage
        .update_vault(
            &vault.id,
            &VaultUpdate {
                name: None,
                description: Some(Some("Updated description".into())),
                icon: None,
            },
        )
        .unwrap();

    let vault_dir = storage.vault_path(&vault);
    let meta = FsStorage::read_vault_json(&vault_dir).unwrap();

    assert_eq!(meta.description.as_deref(), Some("Updated description"));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn update_vault_can_clear_description_in_vault_json() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let vault = Vault {
        id: "vault-1".into(),
        slug: "writing".into(),
        name: "Writing".into(),
        description: Some("Original description".into()),
        owner: VaultOwner::User("user-1".into()),
        permission: NotePermission::Restricted,
        icon: Some("folder".into()),
    };

    storage.create_vault(&vault).unwrap();
    storage
        .update_vault(
            &vault.id,
            &VaultUpdate {
                name: None,
                description: Some(None),
                icon: None,
            },
        )
        .unwrap();

    let vault_dir = storage.vault_path(&vault);
    let meta = FsStorage::read_vault_json(&vault_dir).unwrap();

    assert_eq!(meta.description, None);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn save_and_load_comments_round_trip_with_structured_anchor() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let vault = Vault {
        id: "vault-1".into(),
        slug: "writing".into(),
        name: "Writing".into(),
        description: None,
        owner: VaultOwner::User("user-1".into()),
        permission: NotePermission::Restricted,
        icon: None,
    };

    storage.create_vault(&vault).unwrap();

    let created_at = Utc::now();
    let comments = vec![Comment {
        id: "comment-1".into(),
        note_id: "note-1".into(),
        author_id: "user-1".into(),
        author_name: "User One".into(),
        body: "Please expand this thought.".into(),
        anchor: CommentAnchor {
            text: "selected phrase".into(),
            prefix: "Text before ".into(),
            suffix: " text after".into(),
            range_from: 10,
            range_to: 25,
            attachment: CommentAttachment::Attached,
            line_preview: "A selected line of note text".into(),
            last_matched_at: Some(created_at),
        },
        resolved: false,
        visibility: CommentVisibility::Visible,
        created_at,
        updated_at: created_at,
        replies: vec![CommentReply {
            id: "reply-1".into(),
            author_id: "user-2".into(),
            author_name: "User Two".into(),
            body: "Agreed".into(),
            created_at,
        }],
    }];

    storage
        .save_comments(&vault.id, "note-1", &comments)
        .unwrap();
    let loaded = storage.load_comments(&vault.id, "note-1").unwrap();

    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].anchor.text, "selected phrase");
    assert_eq!(
        loaded[0].anchor.line_preview,
        "A selected line of note text"
    );
    assert!(matches!(loaded[0].visibility, CommentVisibility::Visible));
    assert_eq!(loaded[0].replies.len(), 1);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn load_comments_converts_legacy_quote_only_records_to_hidden_legacy() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let vault = Vault {
        id: "vault-1".into(),
        slug: "writing".into(),
        name: "Writing".into(),
        description: None,
        owner: VaultOwner::User("user-1".into()),
        permission: NotePermission::Restricted,
        icon: None,
    };

    storage.create_vault(&vault).unwrap();

    let vault_dir = storage.vault_path(&vault);
    let legacy_path = vault_dir.join("note-1.comments.json");
    fs::write(
        &legacy_path,
        r#"[
  {
    "id": "comment-legacy",
    "note_id": "note-1",
    "author_id": "user-1",
    "author_name": "User One",
    "body": "Legacy comment",
    "quoted_text": "legacy quote",
    "resolved": false,
    "created_at": "2026-03-28T10:00:00Z",
    "replies": []
  }
]"#,
    )
    .unwrap();

    let loaded = storage.load_comments(&vault.id, "note-1").unwrap();

    assert_eq!(loaded.len(), 1);
    assert!(matches!(
        loaded[0].visibility,
        CommentVisibility::HiddenLegacy
    ));
    assert!(matches!(
        loaded[0].anchor.attachment,
        CommentAttachment::Detached
    ));
    assert_eq!(loaded[0].anchor.text, "legacy quote");
    assert_eq!(loaded[0].anchor.line_preview, "legacy quote");

    fs::remove_dir_all(root).unwrap();
}
