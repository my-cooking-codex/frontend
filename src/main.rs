use leptos::*;

pub(crate) mod contexts;

use contexts::{api::CurrentApi, login::CurrentLogin, toasts::Toasts};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, CurrentLogin::new(cx));
    provide_context(cx, CurrentApi::new(cx));
    provide_context(cx, Toasts::new(cx));

    view! { cx,
        <h1 class="p-4">"Hello World!"</h1>
    }
}
