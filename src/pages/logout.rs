use leptos::*;
use leptos_router::{Redirect, RedirectProps};
use mcc_frontend_core::storage;

use crate::contexts::prelude::{use_api, use_login, CurrentApi, CurrentLogin};

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login(cx);
    let CurrentApi { set_api, .. } = use_api(cx);

    set_api.set_untracked(None);
    set_login.set_untracked(None);
    storage::remove_login_token();

    view! {cx, <Redirect path="/login"/>}
}
