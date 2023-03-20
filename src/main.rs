use leptos::*;
use leptos_router::*;

pub(crate) mod components;
pub(crate) mod contexts;
pub(crate) mod helpers;
pub(crate) mod pages;

use components::toasts::{Toasts, ToastsProps};
use contexts::{api::CurrentApi, login::CurrentLogin, toasts::Toasts as ToastsContext};
use pages::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, CurrentLogin::new(cx));
    provide_context(cx, CurrentApi::new(cx));
    provide_context(cx, ToastsContext::new(cx));

    view! { cx,
        <Toasts/>
        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! {cx, <Home/>} />
                <Route path="/signup" view=move |cx| view! {cx, <Signup/>} />
                <Route path="/login" view=move |cx| view! {cx, <Login/>} />
                <Route path="/logout" view=move |cx| view! {cx, <Logout/>} />
                <Route path="/recipes" view=move |cx| view! {cx, <Recipes/>} />
                <Route path="/recipes/new" view=move |cx| view! {cx, <NewRecipe/>} />
                <Route path="/recipes/:id" view=move |cx| view! {cx, <Recipe/>} />
                <Route path="/recipes/:id/print" view=move |cx| view! {cx, <RecipePrint/>} />
            </Routes>
        </Router>
    }
}
