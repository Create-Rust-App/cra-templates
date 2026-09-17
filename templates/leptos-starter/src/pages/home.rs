//! Home page: welcome plus the interactive counter widget.

use leptos::*;

use crate::components::counter::Counter;

/// Home page content (wrapped in the app shell by the route handler).
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main>
            <h1>"Leptos Starter"</h1>
            <p>
                "Server-rendered Leptos with htmx interactivity. Health: "
                <a href="/api/healthz">"/api/healthz"</a>
            </p>
            <Counter count=0 />
        </main>
    }
}
