use leptos::*;
use leptos_router::use_navigate;

use crate::contexts::prelude::{use_login, CurrentLogin};

pub enum LoginState {
    Unauthenticated,
    Authenticated,
}

/// Redirects to a given path if the user is in an unexpected authentication state.
pub fn login_redirect_effect(cx: Scope, required_state: LoginState, to: String) {
    let navigator = use_navigate(cx);
    let CurrentLogin { login, .. } = use_login(cx);

    create_effect(cx, move |_| {
        let is_authenticated = login.get().is_some();
        match (is_authenticated, &required_state) {
            (true, LoginState::Unauthenticated) => {
                navigator(&to, Default::default()).unwrap();
            }
            (false, LoginState::Authenticated) => {
                navigator(&to, Default::default()).unwrap();
            }
            _ => {
                // do nothing, we're in the right state
            }
        };
    })
}
