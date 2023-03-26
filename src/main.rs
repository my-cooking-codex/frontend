use leptos::*;
use leptos_router::*;

pub(crate) mod components;
pub(crate) mod contexts;
pub(crate) mod helpers;
pub(crate) mod modals;
pub(crate) mod pages;

use components::toasts::{Toasts, ToastsProps};
use contexts::prelude::{
    CurrentApi, CurrentLogin, ModalController, ModalViewer, ModalViewerProps,
    Toasts as ToastsContext,
};
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
    provide_context(cx, ModalController::new(cx));

    view! { cx,
        <Toasts/>
        <ModalViewer/>
        <Router>
            <Routes>
                <Route path="/" view=move |cx| view! {cx, <Home/>} />
                <Route path="/signup" view=move |cx| view! {cx, <Signup/>} />
                <Route path="/login" view=move |cx| view! {cx, <Login/>} />
                <Route path="/logout" view=move |cx| view! {cx, <Logout/>} />
                <Route path="/recipes" view=move |cx| view! {cx, <Recipes/>} />
                <Route path="/recipes/new" view=move |cx| view! {cx, <NewRecipe/>} />
                <Route path="/recipes/:id" view=move |cx| view! {cx, <RecipePage/>} />
                <Route path="/recipes/:id/print" view=move |cx| view! {cx, <RecipePrint/>} />
            </Routes>
        </Router>
    }
}
