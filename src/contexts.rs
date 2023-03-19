pub mod api;
pub mod login;
pub mod toasts;

/// Module used to re-export frequently used items, to reduce imports.
pub mod prelude {
    pub use super::api::{use_api, CurrentApi};
    pub use super::login::{use_login, CurrentLogin};
    pub use super::toasts::{use_toasts, Toast, Toasts};
}
