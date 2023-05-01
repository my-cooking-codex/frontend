use leptos::*;
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    /// Message to display to user
    pub message: String,
}

#[derive(Copy, Clone)]
pub struct Toasts {
    pub toasts: ReadSignal<VecDeque<Toast>>,
    set_toasts: WriteSignal<VecDeque<Toast>>,
}

impl Toasts {
    pub fn new(cx: Scope) -> Self {
        let (toasts, set_toasts) = create_signal(cx, VecDeque::<Toast>::default());
        Self { toasts, set_toasts }
    }

    /// Method to push a "push toast" change
    pub fn push(&self, toast: Toast) {
        self.set_toasts.update(|v| v.push_back(toast.clone()));
        log::debug!("pushed toast: {:?}", toast);
    }

    /// Method to push a "remove toast" change
    pub fn remove(&self, toast: Toast) {
        self.set_toasts.update(|toasts| {
            if let Some(i) = toasts.iter().position(|v| *v == toast) {
                toasts.remove(i).unwrap();
            };
        })
    }
}

pub fn use_toasts(cx: Scope) -> Toasts {
    use_context::<Toasts>(cx).expect("unable to get current login context")
}

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
pub fn ToastsViewer(cx: Scope) -> impl IntoView {
    let Toasts { toasts, .. } = use_toasts(cx);

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
