//! Slug helper: URL-safe identifiers from display names.

/// Convert `text` to a lowercase `kebab-case` slug.
///
/// Splits on non-alphanumeric runs, drops empty segments, and joins the rest
/// with `-`. Returns an empty string when nothing alphanumeric remains.
pub fn slugify(text: &str) -> String {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugifies_display_names() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("  axum_starter 2 "), "axum-starter-2");
        assert_eq!(slugify("---"), "");
    }
}
