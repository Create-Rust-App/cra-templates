//! Counter widget: server-rendered value, htmx-driven increments.
//!
//! The button posts the current count to `/api/count`; the endpoint answers
//! with a refreshed [`Counter`] fragment that htmx swaps in place — no WASM
//! required.

use leptos::*;

/// Counter widget showing `count` with an increment button.
#[component]
pub fn Counter(count: i32) -> impl IntoView {
    let vals = format!(r#"{{"count": {count}}}"#);
    view! {
        <div id="counter">
            <p>"Count: " <span id="count">{count}</span></p>
            <button
                hx-post="/api/count"
                hx-target="#counter"
                hx-swap="outerHTML"
                hx-vals=vals
            >
                "Increment"
            </button>
        </div>
    }
}
