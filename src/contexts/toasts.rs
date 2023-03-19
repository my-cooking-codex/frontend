use std::collections::VecDeque;

use leptos::*;

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
