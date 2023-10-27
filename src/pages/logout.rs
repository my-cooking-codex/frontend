use leptos::*;
use leptos_router::Redirect;

use crate::contexts::prelude::{use_login, CurrentLogin};

#[component]
pub fn Logout() -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login();

    set_login.set(None);

    view! { <Redirect path="/login"/>}
}
