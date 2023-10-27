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
    mount_to_body(|| view! {   <App/> })
}

#[component]
pub fn AppMain() -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home"),
        DrawerLink::new("/recipes", "Recipes"),
        DrawerLink::new("/pantry", "Pantry"),
    ];
    view! {
        <Drawer links=drawer_links>
            <Outlet />
        </Drawer>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(CurrentLogin::new());
    provide_context(CurrentApi::new());
    provide_context(Toasts::new());
    provide_context(ModalController::new());

    let has_auth = move || {
        let CurrentLogin { login, .. } = use_login();
        login.get().is_some()
    };

    let has_no_auth = move || {
        let CurrentLogin { login, .. } = use_login();
        login.get().is_none()
    };

    view! {
        <ToastsViewer/>
        <ModalViewer/>
        <Router>
            <Routes>
                <ProtectedRoute path="/" redirect_path="/login" condition=has_auth view=move || view! {<AppMain/>}>
                    <Route path="/" view=move || view! { <Home/>}/>
                    <Route path="/recipes" view=move || view! {<Recipes/>} />
                    <Route path="recipes/:id" view=move || view! {<RecipePage/>} />
                    <Route path="/pantry" view=move || view! {<Pantry/>} />
                </ProtectedRoute>
                <ProtectedRoute path="recipes/:id/print" redirect_path="/login" condition=has_auth view=move || view! {<RecipePrint/>} />
                <ProtectedRoute path="/signup" redirect_path="/" condition=has_no_auth view=move || view! {<Signup/>} />
                <ProtectedRoute path="/login" redirect_path="/" condition=has_no_auth view=move || view! {<Login/>} />
                <Route path="/logout" view=move || view! {<Logout/>} />
            </Routes>
        </Router>
    }
}
