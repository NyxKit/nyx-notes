use chrono::Utc;

use crate::{
    distill_markdown_description, Comment, CommentAnchor, CommentAttachment, CommentVisibility,
};

#[test]
fn distills_first_actual_paragraph() {
    let content = "# Heading\n\n- item\n\nFirst real paragraph here.\nStill same paragraph.\n\nSecond paragraph.";
    let description = distill_markdown_description(content);

    assert_eq!(
        description.as_deref(),
        Some("First real paragraph here. Still same paragraph.")
    );
}

#[test]
fn returns_none_when_no_paragraph_exists() {
    let content = "# Heading\n\n- item\n\n1. ordered item";
    assert_eq!(distill_markdown_description(content), None);
}

#[test]
fn comment_anchor_round_trip_preserves_structured_fields() {
    let now = Utc::now();
    let comment = Comment {
        id: "comment-1".into(),
        note_id: "note-1".into(),
        author_id: "user-1".into(),
        author_name: "User One".into(),
        body: "Please clarify this line.".into(),
        anchor: CommentAnchor {
            text: "selected phrase".into(),
            prefix: "Text before ".into(),
            suffix: " text after".into(),
            range_from: 4,
            range_to: 19,
            attachment: CommentAttachment::Attached,
            line_preview: "A selected line of note text".into(),
            last_matched_at: Some(now),
        },
        resolved: false,
        visibility: CommentVisibility::Visible,
        created_at: now,
        updated_at: now,
        replies: Vec::new(),
    };

    let json = serde_json::to_string(&comment).unwrap();
    let round_trip: Comment = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.anchor.text, "selected phrase");
    assert_eq!(
        round_trip.anchor.line_preview,
        "A selected line of note text"
    );
    assert!(matches!(
        round_trip.anchor.attachment,
        CommentAttachment::Attached
    ));
    assert!(matches!(round_trip.visibility, CommentVisibility::Visible));
}
