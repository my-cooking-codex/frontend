use leptos::*;

#[component]
pub fn ModalNeutral<F>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] close_text: String,
    #[prop(into)] loading: Signal<bool>,
    on_close: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {cx,
        <div class="modal modal-open">
            <div class="modal-box">
                <span class="font-bold text-lg mb-3">{title}</span>
                {children(cx)}
                <div class="modal-action">
                    <button
                        on:click=move |_| on_close()
                        type="button"
                        class="btn"
                        class:loading=move || loading.get()
                    >
                        {close_text}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ModalFormBase<P, S, N>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] positive_text: String,
    #[prop(into, optional)] positive_secondary_text: Option<String>,
    #[prop(into)] negative_text: String,
    #[prop(into)] loading: Signal<bool>,
    on_positive: P,
    on_positive_secondary: S,
    on_negative: N,
    children: Children,
) -> impl IntoView
where
    P: Fn() + 'static,
    S: Fn() + 'static,
    N: Fn() + 'static,
{
    view! {cx,
        <div class="modal modal-open">
            <form class="modal-box" on:submit=move |event| { event.prevent_default(); on_positive(); }>
                <span class="font-bold text-lg mb-3">{title}</span>
                {children(cx)}
                <div class="modal-action">
                    <div class="join">
                        <button
                            type="submit"
                            class="btn btn-primary join-item"
                            class:loading=move || loading.get()
                        >
                            {positive_text}
                        </button>
                        {
                            if let Some(positive_secondary_text) = positive_secondary_text {
                                Some(view!{cx,
                                    <button
                                        on:click=move |_| on_positive_secondary()
                                        type="button"
                                        class="btn btn-secondary join-item"
                                        class:loading=move || loading.get()
                                    >
                                        {positive_secondary_text}
                                    </button>
                                })
                            } else {
                                None
                            }
                        }
                        <button
                            on:click=move |_| on_negative()
                            type="button"
                            class="btn join-item"
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
pub fn ModalSaveCancel<S, C>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] loading: Signal<bool>,
    on_save: S,
    on_cancel: C,
    children: Children,
) -> impl IntoView
where
    S: Fn() + 'static,
    C: Fn() + 'static,
{
    view! {cx,
        <ModalFormBase
            title={title}
            positive_text="Save"
            negative_text="Cancel"
            loading={loading}
            on_positive={on_save}
            on_positive_secondary=||{}
            on_negative={on_cancel}
        >
            {children(cx)}
        </ModalFormBase>
    }
}

#[component]
pub fn ModalCreateCancel<S, C>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] loading: Signal<bool>,
    on_creation: S,
    on_cancel: C,
    children: Children,
) -> impl IntoView
where
    S: Fn() + 'static,
    C: Fn() + 'static,
{
    view! {cx,
        <ModalFormBase
            title={title}
            positive_text="Create"
            negative_text="Cancel"
            loading={loading}
            on_positive={on_creation}
            on_positive_secondary=||{}
            on_negative={on_cancel}
        >
            {children(cx)}
        </ModalFormBase>
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CreationMode {
    Create,
    CreateAndEdit,
}

#[component]
pub fn ModalCreateWithModeCancel<S, C>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] loading: Signal<bool>,
    on_creation: S,
    on_cancel: C,
    children: Children,
) -> impl IntoView
where
    S: Fn(CreationMode) + 'static + Copy,
    C: Fn() + 'static,
{
    view! {cx,
        <ModalFormBase
            title={title}
            positive_text="Create"
            positive_secondary_text="Create & Edit"
            negative_text="Cancel"
            loading={loading}
            on_positive=move || on_creation(CreationMode::Create)
            on_positive_secondary=move || on_creation(CreationMode::CreateAndEdit)
            on_negative={on_cancel}
        >
            {children(cx)}
        </ModalFormBase>
    }
}
