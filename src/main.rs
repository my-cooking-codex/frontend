use leptos::*;
use leptos_router::*;

pub(crate) mod components;
pub(crate) mod contexts;
pub(crate) mod helpers;
pub(crate) mod modals;
pub(crate) mod pages;

use components::drawer::*;
use contexts::prelude::*;
use pages::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
pub fn AppMain(cx: Scope) -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home"),
        DrawerLink::new("/recipes", "Recipes"),
        DrawerLink::new("/pantry", "Pantry"),
    ];
    view! {cx,
        <Drawer links=drawer_links>
            <Outlet />
        </Drawer>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, CurrentLogin::new(cx));
    provide_context(cx, CurrentApi::new(cx));
    provide_context(cx, Toasts::new(cx));
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
        <ToastsViewer/>
        <ModalViewer/>
        <Router>
            <Routes>
                <ProtectedRoute path="/" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <AppMain/>}>
                    <Route path="/" view=move |cx| view! {cx, <Home/>}/>
                    <Route path="/recipes" view=move |cx| view! {cx, <Recipes/>} />
                    <Route path="recipes/:id" view=move |cx| view! {cx, <RecipePage/>} />
                    <Route path="/pantry" view=move |cx| view! {cx, <Pantry/>} />
                </ProtectedRoute>
                <ProtectedRoute path="recipes/:id/print" redirect_path="/login" condition=has_auth view=move |cx| view! {cx, <RecipePrint/>} />
                <ProtectedRoute path="/signup" redirect_path="/" condition=has_no_auth view=move |cx| view! {cx, <Signup/>} />
                <ProtectedRoute path="/login" redirect_path="/" condition=has_no_auth view=move |cx| view! {cx, <Login/>} />
                <Route path="/logout" view=move |cx| view! {cx, <Logout/>} />
            </Routes>
        </Router>
    }
}
