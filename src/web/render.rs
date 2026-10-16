//! Server-side Markdown to HTML rendering using pulldown-cmark.
//!
//! Enforces raw HTML escaping for security and transforms code blocks
//! into magazine-style code panels with copy buttons and language headers.

use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/// Render a Markdown snippet to sanitized, magazine-styled HTML.
pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut custom_events = Vec::new();

    for event in parser {
        match event {
            // Convert raw HTML into plain text so pulldown_cmark safely escapes it
            Event::Html(raw) => {
                custom_events.push(Event::Text(raw));
            }
            Event::InlineHtml(raw) => {
                custom_events.push(Event::Text(raw));
            }
            // Intercept code block start
            Event::Start(Tag::CodeBlock(ref kind)) => {
                let lang_label = match kind {
                    CodeBlockKind::Fenced(info) => {
                        let trimmed = info.trim();
                        if trimmed.is_empty() {
                            "code"
                        } else {
                            trimmed
                        }
                    }
                    CodeBlockKind::Indented => "code",
                };

                let header_html = format!(
                    "<div class=\"code-wrapper\">\
                     <div class=\"code-header\">\
                       <span class=\"code-lang\">{}</span>\
                       <button type=\"button\" class=\"copy-btn\">Copy</button>\
                     </div>\
                     <pre class=\"code-block\"><code>",
                    html_escape(lang_label)
                );
                custom_events.push(Event::Html(header_html.into()));
            }
            // Intercept code block end
            Event::End(TagEnd::CodeBlock) => {
                custom_events.push(Event::Html("</code></pre></div>".into()));
            }
            other => {
                custom_events.push(other);
            }
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, custom_events.into_iter());
    html_output
}

/// Helper to escape special HTML characters.
pub fn html_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_raw_html() {
        let input = "<script>alert('xss')</script>";
        let rendered = render_markdown(input);
        assert!(!rendered.contains("<script>"));
        assert!(rendered.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_code_block_wrapping() {
        let input = "```rust\nfn main() {}\n```";
        let rendered = render_markdown(input);
        assert!(rendered.contains("class=\"code-wrapper\""));
        assert!(rendered.contains("class=\"code-header\""));
        assert!(rendered.contains("class=\"copy-btn\""));
        assert!(rendered.contains("<span class=\"code-lang\">rust</span>"));
        assert!(rendered.contains("fn main() {}"));
    }
}
