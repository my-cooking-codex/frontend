use crate::{
    components::drawer::*,
    helpers::{login_redirect_effect, LoginState},
};
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home", true),
        DrawerLink::new("/recipes/new", "New Recipe", false),
        DrawerLink::new("/recipes", "Recipes", false),
    ];

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    view! {cx,
        <Drawer links={drawer_links}>
            <div class="p-4 rounded bg-base-200">
                <h1 class="text-3xl font-bold mb-2">"Home"</h1>
            </div>
        </Drawer>
    }
}
