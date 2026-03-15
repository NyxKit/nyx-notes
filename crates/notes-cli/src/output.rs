use chrono::{DateTime, Utc};
use comfy_table::{ContentArrangement, Table};
use notes_core::NoteMeta;

pub fn relative_time(dt: &DateTime<Utc>) -> String {
    let secs = Utc::now().signed_duration_since(*dt).num_seconds();
    match secs {
        s if s < 60 => "just now".into(),
        s if s < 3_600 => format!("{} min ago", s / 60),
        s if s < 86_400 => format!("{} hr ago", s / 3_600),
        s if s < 172_800 => "yesterday".into(),
        s => format!("{} days ago", s / 86_400),
    }
}

pub fn print_notes_table(notes: &[NoteMeta]) {
    if notes.is_empty() {
        println!("(no notes)");
        return;
    }
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["ID", "TITLE", "TAGS", "UPDATED"]);
    for meta in notes {
        table.add_row(vec![
            meta.id.clone(),
            meta.title.clone(),
            meta.tags.join(", "),
            relative_time(&meta.updated_at),
        ]);
    }
    println!("{table}");
}
