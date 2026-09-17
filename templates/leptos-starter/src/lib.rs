//! Leptos fullstack starter (server-side rendering + htmx interactivity).
//!
//! Feature-based layout: pages own routes, components own UI fragments, and
//! API modules own JSON/HTML endpoints. The app shell lives in [`app`];
//! runtime configuration in [`config`].

pub mod api;
pub mod app;
pub mod components;
pub mod config;
pub mod pages;
