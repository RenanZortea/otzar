use comrak::{
    nodes::NodeValue,
    parse_document,
    Arena,
    Options,
    markdown_to_html,
};

/// Render markdown to HTML using Comrak with native math enabled.
pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::default();

    // Extensions (use the exact Extension field names from the docs)
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.footnotes = true;
    options.extension.inline_footnotes = true; // requires `footnotes`
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.superscript = true;
    // To enable header IDs (anchor links), set header_ids to Some(prefix)
    // options.extension.header_ids = Some("user-content-".to_string());

    // Native math parsing
    options.extension.math_dollars = true;
    options.extension.math_code = true;

    // Render-time options: allow raw HTML if you intend to pass it through later
    options.render.r#unsafe = true;

    markdown_to_html(markdown, &options)
}

/// Extract plain text from markdown while preserving math literal content.
pub fn extract_plain_text(markdown: &str) -> String {
    let mut options = Options::default();

    // Match parse behavior used for rendering
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.footnotes = true;
    options.extension.inline_footnotes = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.superscript = true;
    options.extension.math_dollars = true;
    options.extension.math_code = true;

    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &options);

    let mut text = String::new();

    for node in root.descendants() {
        match &node.data.borrow().value {
            NodeValue::Text(t) => text.push_str(t.as_ref()),
            NodeValue::Code(c) => text.push_str(&c.literal),
            NodeValue::Math(m) => text.push_str(&m.literal),
            NodeValue::SoftBreak | NodeValue::LineBreak => text.push(' '),
            NodeValue::HtmlInline(s) | NodeValue::Raw(s) => text.push_str(s),
            _ => {}
        }
    }

    // Normalize whitespace
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n**bold** and *italic*";
        let html = render_markdown(md);
        assert!(html.contains("<h1"));
        assert!(html.contains("<strong>"));
        assert!(html.contains("<em>"));
    }

    #[test]
    fn test_task_list() {
        let md = "- [ ] Todo\n- [x] Done";
        let html = render_markdown(md);
        assert!(html.contains("checkbox") || html.contains("<input"));
    }

    #[test]
    fn test_extract_text() {
        let md = "**Hello** _world_!";
        let text = extract_plain_text(md);
        assert_eq!(text, "Hello world!");
    }

    #[test]
    fn test_math_pass_through() {
        let md = "Inline math: $E=mc^2$\n\n$$\\int_0^\\infty e^{-x^2} dx$$";
        let html = render_markdown(md);
        assert!(html.contains("E=mc^2") || html.contains("\\int_0"));
    }

    #[test]
    fn test_extract_math_text() {
        let md = "Equation inline $a + b$ and display $$c + d$$";
        let text = extract_plain_text(md);
        assert!(text.contains("a + b"));
        assert!(text.contains("c + d"));
    }

#[test]
fn test_math_structure() {
    let html = render_markdown("$x + y$");
    assert!(html.contains(r#"class="math""#), "Comrak parsed math but didn't output math span");
}

}

