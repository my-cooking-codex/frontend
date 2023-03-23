use leptos::*;

use crate::{
    contexts::prelude::{use_api, use_login, CurrentApi, CurrentLogin},
    helpers::{login_redirect_effect, LoginState},
};

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login(cx);
    let CurrentApi { set_api, .. } = use_api(cx);

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    create_effect(cx, move |_| {
        set_api.set(None);
        set_login.set(None);
    });

    view! {cx, <></>}
}
