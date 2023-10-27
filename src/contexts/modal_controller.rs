use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModalController {
    modal: RwSignal<Option<View>>,
    pub current_modal: ReadSignal<Option<View>>,
}

impl ModalController {
    pub fn new() -> Self {
        let modal = create_rw_signal(None);
        Self {
            modal,
            current_modal: modal.read_only(),
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
}

#[component]
pub fn ModalViewer() -> impl IntoView {
    let modal_controller = use_modal_controller();
    view! {
        {move || {
            if let Some(modal) = modal_controller.current_modal.get() {
                #[allow(unused_braces)]
                return Some(view! { {modal}});
            }
            None
        }}
    }
}

pub fn use_modal_controller() -> ModalController {
    use_context().expect("unable to get current modal context")
}
