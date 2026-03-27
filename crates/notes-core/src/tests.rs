use crate::distill_markdown_description;

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
