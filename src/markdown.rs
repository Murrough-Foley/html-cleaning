//! HTML to Markdown conversion.
//!
//! Convert HTML documents to Markdown with structure preservation.
//!
//! Full implementation will be added in Story 10.

use std::fmt::Write;

use dom_query::{Document, Selection};

/// Configuration for Markdown output.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct MarkdownOptions {
    /// Preserve heading hierarchy (h1-h6 → #-######)
    pub preserve_headings: bool,
    /// Include links as `[text](url)`
    pub include_links: bool,
    /// Include images as `![alt](src)`
    pub include_images: bool,
    /// Preserve emphasis (bold, italic)
    pub preserve_emphasis: bool,
    /// Preserve lists (ul/ol → -/1.)
    pub preserve_lists: bool,
    /// Preserve code blocks and inline code
    pub preserve_code: bool,
    /// Preserve blockquotes
    pub preserve_blockquotes: bool,
    /// Preserve tables (if simple structure)
    pub preserve_tables: bool,
    /// Maximum heading level to preserve (1-6)
    pub max_heading_level: u8,
    /// Line width for wrapping (0 = no wrap)
    pub line_width: usize,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            preserve_headings: true,
            include_links: true,
            include_images: true,
            preserve_emphasis: true,
            preserve_lists: true,
            preserve_code: true,
            preserve_blockquotes: true,
            preserve_tables: false,
            max_heading_level: 6,
            line_width: 0,
        }
    }
}

/// Convert HTML document to Markdown.
///
/// Uses default options.
///
/// # Example
///
/// ```
/// use html_cleaning::markdown::to_markdown;
/// use dom_query::Document;
///
/// let doc = Document::from("<h1>Title</h1><p>Content</p>");
/// let md = to_markdown(&doc);
/// assert!(md.contains("# Title"));
/// ```
#[must_use]
pub fn to_markdown(doc: &Document) -> String {
    to_markdown_with_options(doc, &MarkdownOptions::default())
}

/// Convert HTML document to Markdown with options.
#[must_use]
pub fn to_markdown_with_options(doc: &Document, options: &MarkdownOptions) -> String {
    let mut output = String::new();
    let body = doc.select("body");

    if body.exists() {
        for node in body.children().nodes() {
            let sel = Selection::from(*node);
            convert_node(&sel, &mut output, options, 0);
        }
    } else {
        // No body, process root
        for node in doc.select("*").first().children().nodes() {
            let sel = Selection::from(*node);
            convert_node(&sel, &mut output, options, 0);
        }
    }

    normalize_output(&output)
}

/// Convert HTML element to Markdown.
#[must_use]
pub fn element_to_markdown(element: &Selection) -> String {
    element_to_markdown_with_options(element, &MarkdownOptions::default())
}

/// Convert HTML element to Markdown with options.
#[must_use]
pub fn element_to_markdown_with_options(element: &Selection, options: &MarkdownOptions) -> String {
    let mut output = String::new();
    convert_node(element, &mut output, options, 0);
    normalize_output(&output)
}

/// Convert HTML string to Markdown.
///
/// # Example
///
/// ```
/// use html_cleaning::markdown::html_to_markdown;
///
/// let md = html_to_markdown("<h1>Title</h1><p>Content</p>");
/// assert!(md.contains("# Title"));
/// assert!(md.contains("Content"));
/// ```
#[must_use]
pub fn html_to_markdown(html: &str) -> String {
    let doc = Document::from(html);
    to_markdown(&doc)
}

/// Convert HTML string to Markdown with options.
#[must_use]
pub fn html_to_markdown_with_options(html: &str, options: &MarkdownOptions) -> String {
    let doc = Document::from(html);
    to_markdown_with_options(&doc, options)
}

fn get_tag_name(sel: &Selection) -> String {
    sel.nodes()
        .first()
        .and_then(dom_query::NodeRef::node_name)
        .unwrap_or_default()
        .to_lowercase()
}

fn convert_node(sel: &Selection, output: &mut String, options: &MarkdownOptions, depth: usize) {
    let tag = get_tag_name(sel);

    match tag.as_str() {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" if options.preserve_headings => {
            if let Some(level) = tag.chars().nth(1).and_then(|c| c.to_digit(10)) {
                let level = level as usize;
                if level <= options.max_heading_level as usize {
                    output.push_str(&"#".repeat(level));
                    output.push(' ');
                    convert_inline_content(sel, output, options);
                    output.push_str("\n\n");
                }
            }
        }
        "p" => {
            convert_inline_content(sel, output, options);
            output.push_str("\n\n");
        }
        "br" => {
            output.push_str("  \n");
        }
        "hr" => {
            output.push_str("\n---\n\n");
        }
        "ul" if options.preserve_lists => {
            convert_list(sel, output, options, false, depth);
            output.push('\n');
        }
        "ol" if options.preserve_lists => {
            convert_list(sel, output, options, true, depth);
            output.push('\n');
        }
        "blockquote" if options.preserve_blockquotes => {
            convert_blockquote(sel, output, options);
        }
        "pre" if options.preserve_code => {
            convert_code_block(sel, output);
        }
        "code" if options.preserve_code => {
            output.push('`');
            output.push_str(sel.text().as_ref());
            output.push('`');
        }
        "strong" | "b" if options.preserve_emphasis => {
            output.push_str("**");
            convert_inline_content(sel, output, options);
            output.push_str("**");
        }
        "em" | "i" if options.preserve_emphasis => {
            output.push('*');
            convert_inline_content(sel, output, options);
            output.push('*');
        }
        "a" if options.include_links => {
            let href = sel.attr("href").unwrap_or_default();
            output.push('[');
            convert_inline_content(sel, output, options);
            output.push_str("](");
            output.push_str(&href);
            output.push(')');
        }
        "img" if options.include_images => {
            let src = sel.attr("src").unwrap_or_default();
            let alt = sel.attr("alt").unwrap_or_default();
            output.push_str("![");
            output.push_str(&alt);
            output.push_str("](");
            output.push_str(&src);
            output.push(')');
        }
        _ => {
            // Recurse into children for unknown elements
            for node in sel.children().nodes() {
                let child = Selection::from(*node);
                convert_node(&child, output, options, depth);
            }
        }
    }
}

fn convert_inline_content(sel: &Selection, output: &mut String, options: &MarkdownOptions) {
    // Get text content, handling inline elements
    if let Some(node) = sel.nodes().first() {
        for child in node.children() {
            if child.is_text() {
                output.push_str(&child.text());
            } else if child.is_element() {
                let child_sel = Selection::from(child);
                convert_node(&child_sel, output, options, 0);
            }
        }
    }
}

fn convert_list(
    sel: &Selection,
    output: &mut String,
    options: &MarkdownOptions,
    ordered: bool,
    depth: usize,
) {
    let indent = "  ".repeat(depth);
    let mut index = 1;

    for node in sel.children().nodes() {
        let child = Selection::from(*node);
        let tag = get_tag_name(&child);

        if tag == "li" {
            output.push_str(&indent);
            if ordered {
                let _ = write!(output, "{index}. ");
                index += 1;
            } else {
                output.push_str("- ");
            }
            convert_inline_content(&child, output, options);
            output.push('\n');

            // Handle nested lists
            for nested in child.children().nodes() {
                let nested_sel = Selection::from(*nested);
                let nested_tag = get_tag_name(&nested_sel);
                if nested_tag == "ul" || nested_tag == "ol" {
                    convert_list(&nested_sel, output, options, nested_tag == "ol", depth + 1);
                }
            }
        }
    }
}

fn convert_blockquote(sel: &Selection, output: &mut String, options: &MarkdownOptions) {
    let text = sel.text().to_string();
    for line in text.lines() {
        output.push_str("> ");
        output.push_str(line.trim());
        output.push('\n');
    }
    output.push('\n');
    let _ = options; // Silence unused warning for now
}

fn convert_code_block(sel: &Selection, output: &mut String) {
    // Check for language hint in class
    let code = sel.select("code");
    let lang = code
        .attr("class")
        .and_then(|c| {
            c.split_whitespace()
                .find(|cls| cls.starts_with("language-"))
                .map(|cls| cls.strip_prefix("language-").unwrap_or("").to_string())
        })
        .unwrap_or_default();

    output.push_str("```");
    output.push_str(&lang);
    output.push('\n');
    output.push_str(sel.text().as_ref());
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n\n");
}

fn normalize_output(text: &str) -> String {
    // Collapse multiple blank lines to two
    let mut result = String::new();
    let mut prev_blank = false;
    let mut blank_count = 0;

    for line in text.lines() {
        let is_blank = line.trim().is_empty();

        if is_blank {
            blank_count += 1;
            if blank_count <= 2 {
                result.push('\n');
            }
            prev_blank = true;
        } else {
            if prev_blank && blank_count > 2 {
                // Already added 2 blank lines
            }
            result.push_str(line);
            result.push('\n');
            prev_blank = false;
            blank_count = 0;
        }
    }

    // Trim trailing whitespace from each line
    result
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
        + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_conversion() {
        let md = html_to_markdown("<h1>Title</h1><h2>Subtitle</h2>");
        assert!(md.contains("# Title"));
        assert!(md.contains("## Subtitle"));
    }

    #[test]
    fn test_paragraph() {
        let md = html_to_markdown("<p>Hello world</p>");
        assert!(md.contains("Hello world"));
    }

    #[test]
    fn test_bold_and_italic() {
        let md = html_to_markdown("<p><strong>bold</strong> and <em>italic</em></p>");
        assert!(md.contains("**bold**"));
        assert!(md.contains("*italic*"));
    }

    #[test]
    fn test_links() {
        let md = html_to_markdown(r#"<a href="https://example.com">Link</a>"#);
        assert!(md.contains("[Link](https://example.com)"));
    }

    #[test]
    fn test_unordered_list() {
        let md = html_to_markdown("<ul><li>One</li><li>Two</li></ul>");
        assert!(md.contains("- One"));
        assert!(md.contains("- Two"));
    }

    #[test]
    fn test_ordered_list() {
        let md = html_to_markdown("<ol><li>First</li><li>Second</li></ol>");
        assert!(md.contains("1. First"));
        assert!(md.contains("2. Second"));
    }

    #[test]
    fn test_code_block() {
        let md = html_to_markdown("<pre><code>fn main() {}</code></pre>");
        assert!(md.contains("```"));
        assert!(md.contains("fn main()"));
    }

    #[test]
    fn test_blockquote() {
        let md = html_to_markdown("<blockquote>Quote text</blockquote>");
        assert!(md.contains("> Quote text"));
    }
}
