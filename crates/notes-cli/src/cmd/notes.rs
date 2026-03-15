use std::process::Command as Process;

use chrono::Utc;
use notes_core::{Note, NoteMeta, Vault};
use notes_storage_fs::FsStorage;
use uuid::Uuid;

use crate::output::{print_notes_table, relative_time};

pub fn list(
    storage: &FsStorage,
    vault_id: &str,
    tag: Option<&str>,
    category: Option<&str>,
    sort: &str,
    json: bool,
) -> anyhow::Result<()> {
    let mut notes = storage.list_notes(vault_id)?;

    if let Some(t) = tag {
        notes.retain(|n| n.tags.iter().any(|tag| tag == t));
    }
    if let Some(cat) = category {
        notes.retain(|n| n.category.as_deref() == Some(cat));
    }

    match sort {
        "created" => notes.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
        "title" => notes.sort_by(|a, b| a.title.cmp(&b.title)),
        _ => {} // default: updated_at desc (already sorted by FsStorage)
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&notes)?);
    } else {
        print_notes_table(&notes);
    }

    Ok(())
}

pub fn new(
    storage: &FsStorage,
    vault: &Vault,
    user_id: &str,
    title: Option<String>,
    tags: Vec<String>,
    category: Option<String>,
    do_edit: bool,
    editor: &str,
) -> anyhow::Result<()> {
    let title = match title {
        Some(t) => t,
        None => dialoguer::Input::new()
            .with_prompt("Title")
            .interact_text()?,
    };

    let now = Utc::now();
    let note = Note {
        meta: NoteMeta {
            id: Uuid::new_v4().to_string(),
            vault_id: vault.id.clone(),
            title,
            author_id: user_id.to_string(),
            tags,
            category,
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission: vault.permission.clone(),
        },
        content: String::new(),
    };

    storage.save_note(&note)?;
    println!("Created: {}", note.meta.id);

    if do_edit {
        edit(storage, &vault.id, &note.meta.id, editor)?;
    }

    Ok(())
}

pub fn edit(
    storage: &FsStorage,
    vault_id: &str,
    id: &str,
    editor: &str,
) -> anyhow::Result<()> {
    let path = storage.note_file_path(vault_id, id)?;

    let status = Process::new(editor)
        .arg(&path)
        .status()
        .map_err(|e| anyhow::anyhow!("failed to launch '{editor}': {e}"))?;

    if !status.success() {
        anyhow::bail!("editor exited with non-zero status");
    }

    // Re-read the file and touch updated_at so the timestamp reflects the edit.
    let mut note = storage.load_note(vault_id, id)?;
    note.meta.updated_at = Utc::now();
    storage.save_note(&note)?;

    Ok(())
}

pub fn show(note: &Note, raw: bool) -> anyhow::Result<()> {
    if raw {
        // Reserialise with frontmatter so the output matches the on-disk format.
        let yaml = serde_yaml::to_string(&note.meta)
            .map_err(|e| anyhow::anyhow!("serialisation error: {e}"))?;
        print!("---\n{yaml}---\n{}", note.content);
    } else {
        print!("{}", note.content);
    }
    Ok(())
}

pub fn delete(
    storage: &FsStorage,
    vault_id: &str,
    id: &str,
    force: bool,
) -> anyhow::Result<()> {
    if !force {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Delete note '{id}'?"))
            .default(false)
            .interact()?;
        if !confirmed {
            println!("Aborted.");
            return Ok(());
        }
    }

    storage.delete_note(vault_id, id)?;
    println!("Deleted: {id}");
    Ok(())
}

pub fn search(
    storage: &FsStorage,
    vault_id: &str,
    query: &str,
    tag: Option<&str>,
    search_body: bool,
) -> anyhow::Result<()> {
    let query_lower = query.to_lowercase();
    let all_meta = storage.list_notes(vault_id)?;

    let mut results: Vec<NoteMeta> = Vec::new();

    for meta in all_meta {
        let tag_ok = tag.map_or(true, |t| meta.tags.iter().any(|tag| tag == t));
        if !tag_ok {
            continue;
        }

        if meta.title.to_lowercase().contains(&query_lower) {
            results.push(meta);
            continue;
        }

        if search_body {
            if let Ok(note) = storage.load_note(vault_id, &meta.id) {
                if note.content.to_lowercase().contains(&query_lower) {
                    results.push(meta);
                }
            }
        }
    }

    print_notes_table(&results);
    Ok(())
}

pub fn tags(storage: &FsStorage, vault_id: &str) -> anyhow::Result<()> {
    let notes = storage.list_notes(vault_id)?;

    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for meta in &notes {
        for tag in &meta.tags {
            *counts.entry(tag.as_str()).or_insert(0) += 1;
        }
    }

    if counts.is_empty() {
        println!("(no tags)");
    } else {
        for (tag, count) in &counts {
            println!("{tag:<30} {count}");
        }
    }

    Ok(())
}
