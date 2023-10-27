use leptos::*;

#[derive(Debug, Clone, Copy)]
pub enum LoadingItemsState {
    Loaded(usize),
    Loading,
    Failed,
}

#[component]
pub fn BufferedPageLoader<F, R>(
    items_state: Signal<LoadingItemsState>,
    items_per_page: Signal<usize>,
    load_more_action: F,
    retry_action: R,
) -> impl IntoView
where
    F: Fn() + 'static + Copy,
    R: Fn() + 'static + Copy,
{
    view! {
        <div class="flex justify-center">
            {move || {
                match items_state.get() {
                    LoadingItemsState::Loaded(items) => {
                        if items == items_per_page.get() {
                            view!{
                                <button
                                    on:click=move |_| load_more_action()
                                    class="btn btn-block"
                                    type="button"
                                >"More"</button>
                            }.into_any()
                        } else {
                            view! {
                                <div class="text-center"
                                >"Reached Bottom"</div>
                            }.into_any()
                        }
                    },
                    LoadingItemsState::Loading => view!{
                        <div class="loading loading-ring loading-lg"></div>
                    }.into_any(),
                    LoadingItemsState::Failed => view!{
                        <button
                            on:click=move |_| retry_action()
                            class="btn btn-block"
                            type="button"
                        >"More, (Retry)"</button>
                    }.into_any(),
                }
            }}
        </div>
    }
}
