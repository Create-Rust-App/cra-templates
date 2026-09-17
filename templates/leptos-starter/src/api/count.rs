//! Counter fragment endpoint answering htmx increments.

use axum::{response::Html, Json};
use serde::Deserialize;

use crate::components::counter::Counter;

/// Counter state posted by the widget.
#[derive(Debug, Deserialize)]
pub struct CountBody {
    /// Current count shown in the widget.
    pub count: i32,
}

/// Increment the posted count and return a refreshed widget fragment.
pub async fn increment(Json(body): Json<CountBody>) -> Html<String> {
    Html(CounterFragment::render(body.count + 1))
}

/// Render helper kept separate for unit tests.
pub struct CounterFragment;

impl CounterFragment {
    /// Render the counter widget at `count` to an HTML string.
    pub fn render(count: i32) -> String {
        use leptos::*;
        leptos::ssr::render_to_string(move || view! { <Counter count=count /> }).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_embeds_incremented_count() {
        let html = CounterFragment::render(41);
        assert!(html.contains(">41<"), "count rendered, got: {html}");
        assert!(html.contains("hx-post"), "htmx wiring kept");
    }
}
