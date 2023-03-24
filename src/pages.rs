use leptos::*;

pub mod home;
pub mod login;
pub mod logout;
pub mod new_recipe;
pub mod recipes;
pub mod signup;

pub use home::*;
pub use login::*;
pub use logout::*;
pub use new_recipe::*;
pub use recipes::*;
pub use signup::*;

#[component]
pub fn Recipe(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}

#[component]
pub fn RecipePrint(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}
