use notes_auth::SqliteUserStore;
use notes_core::{CreateUserInput, ServerRole, UpdateUserInput};
use tempfile::tempdir;

fn actor(store: &SqliteUserStore) -> notes_core::User {
    store.login_user("admin", "Correct-password1").unwrap()
}

#[test]
fn bootstraps_admin_and_lists_users() {
    let dir = tempdir().unwrap();
    let store = SqliteUserStore::new(dir.path(), "test-server").unwrap();
    store.bootstrap_admin(Some("Correct-password1")).unwrap();

    let admin = actor(&store);
    let users = store.list_users(&admin).unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].username, "admin");
    assert_eq!(users[0].email, "admin@localhost");
    assert!(!users[0].can_delete);
}

#[test]
fn enforces_required_unique_email_and_password_policy() {
    let dir = tempdir().unwrap();
    let store = SqliteUserStore::new(dir.path(), "test-server").unwrap();
    store.bootstrap_admin(Some("Correct-password1")).unwrap();
    let admin = actor(&store);

    let weak = store.create_user(
        &admin,
        CreateUserInput {
            username: "alice".into(),
            email: "alice@example.com".into(),
            display_name: "Alice".into(),
            role: ServerRole::User,
            password: "short".into(),
        },
    );
    assert!(matches!(weak, Err(notes_core::AuthError::Validation(_))));

    let alice = store
        .create_user(
            &admin,
            CreateUserInput {
                username: "alice".into(),
                email: "alice@example.com".into(),
                display_name: "Alice".into(),
                role: ServerRole::User,
                password: "Correct-password1".into(),
            },
        )
        .unwrap();

    assert_eq!(alice.email, "alice@example.com");

    let duplicate_email = store.create_user(
        &admin,
        CreateUserInput {
            username: "bob".into(),
            email: "alice@example.com".into(),
            display_name: "Bob".into(),
            role: ServerRole::User,
            password: "Correct-password1".into(),
        },
    );
    assert!(matches!(
        duplicate_email,
        Err(notes_core::AuthError::Conflict(_))
    ));
}

#[test]
fn blocks_self_demotion_self_delete_and_last_admin_delete() {
    let dir = tempdir().unwrap();
    let store = SqliteUserStore::new(dir.path(), "test-server").unwrap();
    store.bootstrap_admin(Some("Correct-password1")).unwrap();
    let admin = actor(&store);

    let self_demote = store.update_user(
        &admin,
        &admin.id,
        UpdateUserInput {
            email: admin.email.clone(),
            display_name: admin.display_name.clone(),
            role: ServerRole::User,
            password: None,
        },
    );
    assert!(matches!(
        self_demote,
        Err(notes_core::AuthError::Conflict(_))
    ));

    let self_delete = store.delete_user(&admin, &admin.id);
    assert!(matches!(
        self_delete,
        Err(notes_core::AuthError::Conflict(_))
    ));

    let last_admin_delete = store.delete_user(&admin, "missing");
    assert!(matches!(
        last_admin_delete,
        Err(notes_core::AuthError::UserNotFound)
    ));
}

#[test]
fn creates_updates_and_deletes_other_users() {
    let dir = tempdir().unwrap();
    let store = SqliteUserStore::new(dir.path(), "test-server").unwrap();
    store.bootstrap_admin(Some("Correct-password1")).unwrap();
    let admin = actor(&store);

    let created = store
        .create_user(
            &admin,
            CreateUserInput {
                username: "alice".into(),
                email: "alice@example.com".into(),
                display_name: "Alice".into(),
                role: ServerRole::User,
                password: "Correct-password1".into(),
            },
        )
        .unwrap();

    let updated = store
        .update_user(
            &admin,
            &created.id,
            UpdateUserInput {
                email: "alice-updated@example.com".into(),
                display_name: "Alice Updated".into(),
                role: ServerRole::User,
                password: Some("Updated-password1".into()),
            },
        )
        .unwrap();

    assert_eq!(updated.email, "alice-updated@example.com");
    assert_eq!(updated.display_name, "Alice Updated");

    assert!(store.login_user("alice", "Correct-password1").is_err());
    assert!(store.login_user("alice", "Updated-password1").is_ok());

    store.delete_user(&admin, &created.id).unwrap();
    let users = store.list_users(&admin).unwrap();
    assert_eq!(users.len(), 1);
}
