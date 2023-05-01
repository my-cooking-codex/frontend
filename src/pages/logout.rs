use leptos::*;
use leptos_router::{Redirect, RedirectProps};

use crate::contexts::prelude::{use_login, CurrentLogin};

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login(cx);

    set_login.set(None);

    view! {cx, <Redirect path="/login"/>}
}
