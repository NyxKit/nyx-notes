use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Utc;
use notes_core::{
    Comment, CommentAnchor, CommentAttachment, CommentReply, CommentVisibility, Note, NoteMeta,
    NotePermission, StorageBackend, Vault, VaultOwner, VaultUpdate,
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
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "user-1".into(),
        },
        permission: NotePermission::Restricted,
        icon: Some("folder".into()),
    };

    storage.create_vault(&vault).unwrap();
    storage
        .update_vault(
            &vault.owner,
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
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "user-1".into(),
        },
        permission: NotePermission::Restricted,
        icon: Some("folder".into()),
    };

    storage.create_vault(&vault).unwrap();
    storage
        .update_vault(
            &vault.owner,
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
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "user-1".into(),
        },
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

    let owner = vault.owner.clone();
    storage
        .save_comments(&owner, &vault.id, "note-1", &comments)
        .unwrap();
    let loaded = storage.load_comments(&owner, &vault.id, "note-1").unwrap();

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
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "user-1".into(),
        },
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

    let owner = vault.owner.clone();
    let loaded = storage.load_comments(&owner, &vault.id, "note-1").unwrap();

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

#[test]
fn create_server_vault_writes_under_server_namespace() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let vault = Vault {
        id: "vault-2".into(),
        slug: "handbook".into(),
        name: "Handbook".into(),
        description: None,
        owner: VaultOwner::Server {
            server_slug: "main-server".into(),
        },
        permission: NotePermission::Edit,
        icon: None,
    };

    storage.create_vault(&vault).unwrap();

    assert!(root
        .join("main-server")
        .join("vaults")
        .join("handbook")
        .join(".vault.json")
        .is_file());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn sync_all_homes_rewrites_author_ids_to_canonical_user_ids() {
    let root = temp_root();
    let storage = FsStorage::new(&root);
    let vault = Vault {
        id: "vault-3".into(),
        slug: "writing".into(),
        name: "Writing".into(),
        description: None,
        owner: VaultOwner::Home {
            server_slug: "main-server".into(),
            home_slug: "alice".into(),
        },
        permission: NotePermission::Edit,
        icon: None,
    };

    storage.create_vault(&vault).unwrap();
    storage
        .save_note(
            &vault.owner,
            &Note {
                meta: NoteMeta {
                    id: "note-1".into(),
                    vault_id: vault.slug.clone(),
                    title: "Draft".into(),
                    description: None,
                    author_id: "alice".into(),
                    images: vec![],
                    tags: vec![],
                    category: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    is_encrypted: false,
                    permission: NotePermission::Edit,
                    feedback_type: None,
                    app_location: None,
                    storage_path: None,
                    console_output: None,
                    interaction_trail: None,
                },
                content: "Hello".into(),
            },
        )
        .unwrap();

    let result = storage
        .sync_all_homes_author_id(&|username| {
            if username == "alice" {
                Some("user-alice".into())
            } else {
                None
            }
        })
        .unwrap();

    assert_eq!(result.homes_scanned, 1);
    let loaded = storage
        .load_note(&vault.owner, &vault.id, "note-1")
        .unwrap();
    assert_eq!(loaded.meta.author_id, "user-alice");

    fs::remove_dir_all(root).unwrap();
}
