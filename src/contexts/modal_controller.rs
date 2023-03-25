use leptos::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ModalController {
    modal: RwSignal<Option<View>>,
}

impl ModalController {
    pub fn new(cx: Scope) -> Self {
        Self {
            modal: create_rw_signal(cx, None),
        }
    }

    /// Open a new modal, closing existing modal if one exists
    pub fn open(&self, modal: View) {
        self.modal.set(Some(modal));
    }

    /// Close the current modal, if one is open
    pub fn close(&self) {
        self.modal.set(None);
    }

    pub fn reader(&self) -> ReadSignal<Option<View>> {
        self.modal.read_only()
    }
}

#[component]
pub fn ModalViewer(cx: Scope) -> impl IntoView {
    let modal_controller = use_modal_controller(cx);
    let modal = modal_controller.reader();
    view! { cx,
        {move || {
            if let Some(modal) = modal.get() {
                view! {cx, <>{modal}</>}
            } else {
                view! {cx, <></>}
            }
        }}
    }
}

pub fn use_modal_controller(cx: Scope) -> ModalController {
    use_context(cx).expect("unable to get current modal context")
}
