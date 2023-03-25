use leptos::*;

pub mod home;
pub mod login;
pub mod logout;
pub mod new_recipe;
pub mod recipe_print;
pub mod recipes;
pub mod signup;

pub use home::*;
pub use login::*;
pub use logout::*;
pub use new_recipe::*;
pub use recipe_print::*;
pub use recipes::*;
pub use signup::*;

#[component]
pub fn Recipe(cx: Scope) -> impl IntoView {
    view! {cx, <></>}
}
