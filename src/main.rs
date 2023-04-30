use leptos::*;
use leptos_router::*;

pub(crate) mod components;
pub(crate) mod contexts;
pub(crate) mod helpers;
pub(crate) mod modals;
pub(crate) mod pages;

use components::toasts::{Toasts, ToastsProps};
use contexts::prelude::{
    use_login, CurrentApi, CurrentLogin, ModalController, ModalViewer, ModalViewerProps,
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

    let has_auth = move |cx| {
        let CurrentLogin { login, .. } = use_login(cx);
        login.get().is_some()
    };

    let has_no_auth = move |cx| {
        let CurrentLogin { login, .. } = use_login(cx);
        login.get().is_none()
    };

    view! { cx,
        <Toasts/>
        <ModalViewer/>
        <Router>
            <Routes>
                <ProtectedRoute path="/" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <Home/>}/>
                <ProtectedRoute path="logout" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <Logout/>} />
                <ProtectedRoute path="/recipes" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <Recipes/>} />
                <ProtectedRoute path="recipes/new" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <NewRecipe/>} />
                <ProtectedRoute path="recipes/:id" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <RecipePage/>} />
                <ProtectedRoute path="recipes/:id/print" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <RecipePrint/>} />
                <ProtectedRoute path="/signup" redirect_path="/" condition=has_no_auth view=move |cx| view! {cx, <Signup/>} />
                <ProtectedRoute path="/login" redirect_path="/" condition=has_no_auth view=move |cx| view! {cx, <Login/>} />
            </Routes>
        </Router>
    }
}
