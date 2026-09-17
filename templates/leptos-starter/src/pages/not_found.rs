//! Not-found page for unknown routes.

use leptos::*;

/// 404 page content (wrapped in the app shell by the fallback handler).
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main>
            <h1>"Not found"</h1>
            <p>
                "No page here. " <a href="/">"Back home"</a>
            </p>
        </main>
    }
}
