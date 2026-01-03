//! Integration tests for html-cleaning crate.

use dom_query::Document;
use html_cleaning::{CleaningOptions, HtmlCleaner};

#[test]
fn test_full_cleaning_workflow() {
    let html = r#"
        <html>
        <head>
            <script>console.log('bad');</script>
            <style>body { color: red; }</style>
        </head>
        <body>
            <nav>Navigation</nav>
            <main>
                <h1>Article Title</h1>
                <p>This is the main content.</p>
                <p></p>
                <div class="ad">Advertisement</div>
            </main>
            <footer>Footer content</footer>
        </body>
        </html>
    "#;

    let options = CleaningOptions::builder()
        .remove_tags(&["script", "style", "nav", "footer"])
        .remove_selectors(&[".ad"])
        .prune_empty(true)
        .build();

    let cleaner = HtmlCleaner::with_options(options);
    let doc = Document::from(html);

    cleaner.clean(&doc);

    // Scripts and styles removed
    assert!(doc.select("script").is_empty());
    assert!(doc.select("style").is_empty());

    // Navigation and footer removed
    assert!(doc.select("nav").is_empty());
    assert!(doc.select("footer").is_empty());

    // Ad removed
    assert!(doc.select(".ad").is_empty());

    // Main content preserved
    assert!(doc.select("h1").exists());
    assert!(doc.select("p").exists());
    assert!(doc.text().contains("Article Title"));
    assert!(doc.text().contains("main content"));

    // Empty paragraph should be removed
    let paragraphs = doc.select("p");
    for node in paragraphs.nodes() {
        let sel = dom_query::Selection::from(*node);
        assert!(!sel.text().to_string().trim().is_empty());
    }
}

#[test]
fn test_strip_tags_preserves_content() {
    let html = r#"
        <div>
            <span class="highlight">Important</span> text here.
            <font color="red">Styled</font> content.
        </div>
    "#;

    let options = CleaningOptions {
        tags_to_strip: vec!["span".into(), "font".into()],
        ..Default::default()
    };

    let cleaner = HtmlCleaner::with_options(options);
    let doc = Document::from(html);

    cleaner.clean(&doc);

    // Tags removed
    assert!(doc.select("span").is_empty());
    assert!(doc.select("font").is_empty());

    // Content preserved
    let text = doc.text().to_string();
    assert!(text.contains("Important"));
    assert!(text.contains("Styled"));
}

#[test]
fn test_attribute_cleaning() {
    let html = r#"
        <div>
            <a href="https://example.com" class="link" id="main-link" onclick="bad()">Link</a>
            <img src="image.jpg" alt="Image" class="photo" data-tracking="xyz">
        </div>
    "#;

    let options = CleaningOptions {
        strip_attributes: true,
        preserved_attributes: vec!["href".into(), "src".into(), "alt".into()],
        ..Default::default()
    };

    let cleaner = HtmlCleaner::with_options(options);
    let doc = Document::from(html);

    cleaner.clean(&doc);

    let link = doc.select("a");
    assert!(link.attr("href").is_some());
    assert!(link.attr("class").is_none());
    assert!(link.attr("onclick").is_none());

    let img = doc.select("img");
    assert!(img.attr("src").is_some());
    assert!(img.attr("alt").is_some());
    assert!(img.attr("class").is_none());
    assert!(img.attr("data-tracking").is_none());
}

#[cfg(feature = "presets")]
mod preset_tests {
    use super::*;
    use html_cleaning::presets;

    #[test]
    fn test_minimal_preset() {
        let html = r#"
            <div>
                <script>alert('bad');</script>
                <style>.x {}</style>
                <p>Content</p>
                <nav>Navigation</nav>
            </div>
        "#;

        let cleaner = HtmlCleaner::with_options(presets::minimal());
        let doc = Document::from(html);

        cleaner.clean(&doc);

        // Script and style removed
        assert!(doc.select("script").is_empty());
        assert!(doc.select("style").is_empty());

        // Nav preserved (minimal doesn't remove it)
        assert!(doc.select("nav").exists());
    }

    #[test]
    fn test_standard_preset() {
        let html = r#"
            <div>
                <script>bad</script>
                <iframe src="ad.html"></iframe>
                <form><input></form>
                <p>Content</p>
            </div>
        "#;

        let cleaner = HtmlCleaner::with_options(presets::standard());
        let doc = Document::from(html);

        cleaner.clean(&doc);

        assert!(doc.select("script").is_empty());
        assert!(doc.select("iframe").is_empty());
        assert!(doc.select("form").is_empty());
        assert!(doc.select("p").exists());
    }

    #[test]
    fn test_aggressive_preset() {
        let html = r#"
            <div>
                <nav>Menu</nav>
                <header>Header</header>
                <main><p class="content">Main content</p></main>
                <aside>Sidebar</aside>
                <footer>Footer</footer>
            </div>
        "#;

        let cleaner = HtmlCleaner::with_options(presets::aggressive());
        let doc = Document::from(html);

        cleaner.clean(&doc);

        // Layout elements removed
        assert!(doc.select("nav").is_empty());
        assert!(doc.select("header").is_empty());
        assert!(doc.select("aside").is_empty());
        assert!(doc.select("footer").is_empty());

        // Main content preserved
        assert!(doc.text().contains("Main content"));

        // Attributes stripped (except preserved ones)
        let p = doc.select("p");
        assert!(p.attr("class").is_none());
    }
}

