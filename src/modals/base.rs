use leptos::*;

#[component]
pub fn ModalBase<F>(
    cx: Scope,
    title: String,
    positive_text: String,
    negative_text: String,
    loading: ReadSignal<bool>,
    on_positive: F,
    on_negative: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {cx,
        <div class="modal modal-open">
            <form class="modal-box" on:submit=move |event| { event.prevent_default(); on_positive(); }>
                <span class="font-bold text-lg mb-3">{title}</span>
                {children(cx)}
                <div class="modal-action">
                    <div class="btn-group">
                        <button
                            type="submit"
                            class="btn btn-primary"
                            class:loading=move || loading.get()
                        >
                            {positive_text}
                        </button>
                        <button
                            on:click=move |_| on_negative()
                            type="button"
                            class="btn"
                            class:loading=move || loading.get()
                        >
                            {negative_text}
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}

#[component]
pub fn ModalSaveCancel<F>(
    cx: Scope,
    title: String,
    loading: ReadSignal<bool>,
    on_save: F,
    on_cancel: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    ModalBase(
        cx,
        ModalBaseProps {
            title,
            positive_text: "Save".to_owned(),
            negative_text: "Cancel".to_owned(),
            loading,
            on_positive: on_save,
            on_negative: on_cancel,
            children,
        },
    )
}
