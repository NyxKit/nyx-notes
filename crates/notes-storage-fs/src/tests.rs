use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use notes_core::{NotePermission, StorageBackend, Vault, VaultOwner, VaultUpdate};

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
