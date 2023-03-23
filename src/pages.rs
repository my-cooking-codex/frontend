use leptos::*;

pub mod login;
pub mod logout;

pub use login::{Login, LoginProps};
pub use logout::{Logout, LogoutProps};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx, <h1>"Home"</h1>}
}

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}

#[component]
pub fn Recipes(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}

#[component]
pub fn Recipe(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}

#[component]
pub fn RecipePrint(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}

#[component]
pub fn NewRecipe(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}
