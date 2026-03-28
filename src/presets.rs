//! Prebuilt cleaning configurations.
//!
//! Ready-to-use presets for common cleaning scenarios.

use crate::options::CleaningOptions;

/// Minimal cleaning - just scripts and styles.
///
/// Removes:
/// - `script`, `style`, `noscript`
///
/// Best for: Quick sanitization, preserving most structure.
///
/// # Example
///
/// ```
/// use html_cleaning::{HtmlCleaner, presets};
///
/// let cleaner = HtmlCleaner::with_options(presets::minimal());
/// ```
#[must_use]
pub fn minimal() -> CleaningOptions {
    CleaningOptions {
        tags_to_remove: vec![
            "script".to_string(),
            "style".to_string(),
            "noscript".to_string(),
        ],
        prune_empty: false,
        normalize_whitespace: false,
        ..Default::default()
    }
}

/// Standard cleaning for web scraping.
///
/// Removes:
/// - `script`, `style`, `noscript`
/// - `form`, `iframe`, `object`, `embed`
/// - `svg`, `canvas`, `video`, `audio`
///
/// Enables:
/// - `prune_empty`
/// - `normalize_whitespace`
///
/// Best for: General web scraping, content display.
///
/// # Example
///
/// ```
/// use html_cleaning::{HtmlCleaner, presets};
///
/// let cleaner = HtmlCleaner::with_options(presets::standard());
/// ```
#[must_use]
pub fn standard() -> CleaningOptions {
    CleaningOptions {
        tags_to_remove: vec![
            "script".to_string(),
            "style".to_string(),
            "noscript".to_string(),
            "form".to_string(),
            "iframe".to_string(),
            "object".to_string(),
            "embed".to_string(),
            "svg".to_string(),
            "canvas".to_string(),
            "video".to_string(),
            "audio".to_string(),
        ],
        prune_empty: true,
        normalize_whitespace: true,
        ..Default::default()
    }
}

/// Aggressive cleaning for maximum content extraction.
///
/// Includes everything in `standard()` plus:
/// - Removes: `nav`, `header`, `footer`, `aside`, `figure`, `figcaption`
/// - Enables: `strip_attributes` (preserves `href`, `src`, `alt`)
///
/// Best for: Text extraction, removing all non-content elements.
///
/// # Example
///
/// ```
/// use html_cleaning::{HtmlCleaner, presets};
///
/// let cleaner = HtmlCleaner::with_options(presets::aggressive());
/// ```
#[must_use]
pub fn aggressive() -> CleaningOptions {
    CleaningOptions {
        tags_to_remove: vec![
            // Standard tags
            "script".to_string(),
            "style".to_string(),
            "noscript".to_string(),
            "form".to_string(),
            "iframe".to_string(),
            "object".to_string(),
            "embed".to_string(),
            "svg".to_string(),
            "canvas".to_string(),
            "video".to_string(),
            "audio".to_string(),
            // Layout tags
            "nav".to_string(),
            "header".to_string(),
            "footer".to_string(),
            "aside".to_string(),
            "figure".to_string(),
            "figcaption".to_string(),
        ],
        prune_empty: true,
        normalize_whitespace: true,
        strip_attributes: true,
        preserved_attributes: vec![
            "href".to_string(),
            "src".to_string(),
            "alt".to_string(),
        ],
        ..Default::default()
    }
}

/// Article extraction preset.
///
/// Optimized for extracting article content:
/// - Removes navigation and layout elements
/// - Strips wrapper tags (`div`, `span`) while preserving content
/// - Removes common advertisement selectors
///
/// Best for: News articles, blog posts, content pages.
///
/// # Example
///
/// ```
/// use html_cleaning::{HtmlCleaner, presets};
///
/// let cleaner = HtmlCleaner::with_options(presets::article_extraction());
/// ```
#[must_use]
pub fn article_extraction() -> CleaningOptions {
    CleaningOptions {
        tags_to_remove: vec![
            "script".to_string(),
            "style".to_string(),
            "noscript".to_string(),
            "form".to_string(),
            "iframe".to_string(),
            "object".to_string(),
            "embed".to_string(),
            "svg".to_string(),
            "canvas".to_string(),
            "video".to_string(),
            "audio".to_string(),
            "nav".to_string(),
            "footer".to_string(),
        ],
        tags_to_strip: vec!["span".to_string(), "div".to_string()],
        selectors_to_remove: vec![
            "[role='navigation']".to_string(),
            "[role='banner']".to_string(),
            "[role='complementary']".to_string(),
            ".advertisement".to_string(),
            ".ads".to_string(),
            "#comments".to_string(),
            ".comments".to_string(),
        ],
        prune_empty: true,
        normalize_whitespace: true,
        empty_tags: vec![
            "div".to_string(),
            "span".to_string(),
            "p".to_string(),
            "section".to_string(),
        ],
        ..Default::default()
    }
}

/// Trafilatura-compatible cleaning preset.
///
/// Matches the tag removal, stripping, and pruning behavior used by
/// rs-trafilatura for web content extraction. Removes 50 non-content tags,
/// strips 18 wrapper tags, and prunes 22 tag types when empty.
///
/// This preset handles pure HTML cleaning only — extraction-specific logic
/// (link density analysis, boilerplate detection, content scoring) is NOT
/// included and should be handled by the extraction pipeline.
///
/// Removes:
/// - Script/style/noscript, forms, iframes, embeds, media elements
/// - Navigation, footer, aside, menus
/// - UI elements (buttons, inputs, selects, dialogs)
/// - Non-content elements (applet, marquee, math, svg, canvas)
///
/// Strips (keeps children):
/// - Wrapper/formatting tags: abbr, acronym, address, bdi, bdo, big, cite,
///   data, dfn, font, hgroup, img, ins, mark, meta, ruby, small, template
///
/// Prunes empty:
/// - p, div, span, h1-h6, blockquote, article, section, main, li, dd, dt,
///   em, i, b, strong, pre, q
///
/// Also: removes HTML comments, normalizes whitespace.
///
/// # Example
///
/// ```
/// use html_cleaning::{HtmlCleaner, presets};
///
/// let cleaner = HtmlCleaner::with_options(presets::trafilatura());
/// ```
#[must_use]
pub fn trafilatura() -> CleaningOptions {
    CleaningOptions {
        tags_to_remove: vec![
            // Important
            "aside".into(), "embed".into(), "footer".into(), "form".into(),
            "head".into(), "iframe".into(), "menu".into(), "object".into(),
            "script".into(),
            // Other content
            "applet".into(), "audio".into(), "canvas".into(), "figure".into(),
            "map".into(), "picture".into(), "svg".into(), "video".into(),
            // Secondary
            "area".into(), "blink".into(), "button".into(), "datalist".into(),
            "dialog".into(), "frame".into(), "frameset".into(), "fieldset".into(),
            "link".into(), "input".into(), "ins".into(), "label".into(),
            "legend".into(), "marquee".into(), "math".into(), "menuitem".into(),
            "nav".into(), "noscript".into(), "optgroup".into(), "option".into(),
            "output".into(), "param".into(), "progress".into(), "rp".into(),
            "rt".into(), "rtc".into(), "select".into(), "source".into(),
            "style".into(), "track".into(), "textarea".into(), "time".into(),
            "use".into(),
        ],
        tags_to_strip: vec![
            "abbr".into(), "acronym".into(), "address".into(), "bdi".into(),
            "bdo".into(), "big".into(), "cite".into(), "data".into(),
            "dfn".into(), "font".into(), "hgroup".into(), "img".into(),
            "ins".into(), "mark".into(), "meta".into(), "ruby".into(),
            "small".into(), "template".into(),
        ],
        prune_empty: true,
        empty_tags: vec![
            "p".into(), "div".into(), "span".into(),
            "h1".into(), "h2".into(), "h3".into(), "h4".into(), "h5".into(), "h6".into(),
            "blockquote".into(), "article".into(), "section".into(), "main".into(),
            "li".into(), "dd".into(), "dt".into(),
            "em".into(), "i".into(), "b".into(), "strong".into(),
            "pre".into(), "q".into(),
        ],
        normalize_whitespace: true,
        remove_comments: true,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_preset() {
        let opts = minimal();
        assert_eq!(opts.tags_to_remove.len(), 3);
        assert!(!opts.prune_empty);
    }

    #[test]
    fn test_standard_preset() {
        let opts = standard();
        assert!(opts.tags_to_remove.len() > 5);
        assert!(opts.prune_empty);
        assert!(opts.normalize_whitespace);
    }

    #[test]
    fn test_aggressive_preset() {
        let opts = aggressive();
        assert!(opts.tags_to_remove.contains(&"nav".to_string()));
        assert!(opts.strip_attributes);
        assert!(opts.preserved_attributes.contains(&"href".to_string()));
    }

    #[test]
    fn test_article_extraction_preset() {
        let opts = article_extraction();
        assert!(!opts.selectors_to_remove.is_empty());
        assert!(opts.tags_to_strip.contains(&"span".to_string()));
    }

    #[test]
    fn test_trafilatura_preset() {
        let opts = trafilatura();
        assert_eq!(opts.tags_to_remove.len(), 50);
        assert_eq!(opts.tags_to_strip.len(), 18);
        assert_eq!(opts.empty_tags.len(), 22);
        assert!(opts.prune_empty);
        assert!(opts.normalize_whitespace);
        assert!(opts.remove_comments);
        assert!(opts.tags_to_remove.contains(&"script".to_string()));
        assert!(opts.tags_to_remove.contains(&"nav".to_string()));
        assert!(opts.tags_to_strip.contains(&"font".to_string()));
        assert!(opts.empty_tags.contains(&"blockquote".to_string()));
    }
}
