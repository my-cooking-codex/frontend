use leptos::*;

pub mod home;
pub mod login;
pub mod logout;
pub mod signup;

pub use home::*;
pub use login::*;
pub use logout::*;
pub use signup::*;

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
