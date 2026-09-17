//! HTML application shell: full document wrapping rendered content.
//!
//! Pages render their body through [`shell`]; interactivity ships via htmx
//! (no WASM build step) with `cargo leptos` as the documented upgrade path
//! to hydrated islands (see `docs/DEPLOYMENT.md`).

/// htmx version served from CDN (verified against unpkg).
pub const HTMX_VERSION: &str = "2.0.8";

/// Wrap rendered `content` in a full HTML document.
pub fn shell(title: &str, content: &str) -> String {
    format!(
        "<!DOCTYPE html>\n\
        <html lang=\"en\">\n\
        <head>\n\
        <meta charset=\"utf-8\" />\n\
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\
        <title>{title}</title>\n\
        <script src=\"https://unpkg.com/htmx.org@{HTMX_VERSION}/dist/htmx.min.js\"></script>\n\
        <style>\n\
        body {{ font-family: system-ui, sans-serif; max-width: 42rem; margin: 2rem auto; padding: 0 1rem; }}\n\
        button {{ font-size: 1rem; padding: 0.4rem 0.9rem; cursor: pointer; }}\n\
        </style>\n\
        </head>\n\
        <body>{content}</body>\n\
        </html>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_includes_htmx_and_content() {
        let page = shell("Test", "<p>hi</p>");
        assert!(page.contains("htmx.min.js"), "htmx script tag");
        assert!(page.contains("<p>hi</p>"), "content wrapped");
        assert!(page.starts_with("<!DOCTYPE html>"), "full document");
    }
}
