//! Page components: one module per route.
//!
//! Pages render into [`crate::app::shell`]. Copy `home.rs` to add a page,
//! declare it here, and route to it in `src/main.rs`.

pub mod home;
pub mod not_found;
