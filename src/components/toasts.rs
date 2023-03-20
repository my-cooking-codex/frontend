use std::time::Duration;

use leptos::*;

use crate::contexts::prelude::{use_toasts, Toast, Toasts as ToastsContext};

#[component]
fn ToastRow(cx: Scope, #[prop(into)] toast: Toast) -> impl IntoView {
    {
        let toast = toast.clone();
        set_timeout(
            move || {
                use_toasts(cx).remove(toast);
            },
            Duration::from_secs(6),
        );
    }
    view! {cx, <div class="alert alert-info"><span>{toast.message}</span></div>}
}

#[component]
pub fn Toasts(cx: Scope) -> impl IntoView {
    let ToastsContext { toasts, .. } = use_toasts(cx);

    view! {cx,
        <div class="toast toast-top toast-start z-[999]">
            // XXX should use a 'For' component here instead as it's more efficient,
            // but it requires a key, which toasts don't have
            {move || toasts.get().iter().map(|toast| {
                view! {cx, <ToastRow toast={toast.clone()}/>}
            }).collect::<Vec<_>>()}
        </div>
    }
}
