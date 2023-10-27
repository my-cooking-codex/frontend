use leptos::*;

use mcc_frontend_types::pantry::{CreateItem, Item};

use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::{CreationMode, ModalCreateWithModeCancel},
};

#[component]
pub fn NewItemModal<F>(on_action: F) -> impl IntoView
where
    F: Fn(Option<(CreationMode, Item)>) + 'static + Copy,
{
    let toasts = use_toasts();
    let CurrentApi { api, .. } = use_api();
    let name = create_rw_signal(String::default());
    let location_id = create_rw_signal(String::default());

    let locations = create_resource(
        || {},
        move |_| async move {
            let api = api.get_untracked().expect("api expected to be set");
            match api.get_pantry_locations().await {
                Ok(v) => v,
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "loading locations"));
                    vec![]
                }
            }
        },
    );

    let new_item = create_action(move |mode: &CreationMode| {
        let mode = *mode;
        let api = api.get_untracked().expect("api expected to be set");
        let name = name.get_untracked();
        let location_id = location_id.get_untracked();
        async move {
            match api
                .post_pantry_item(
                    &location_id,
                    &CreateItem {
                        name,
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(v) => on_action(Some((mode, v))),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "creating new item"));
                }
            }
        }
    });

    let global_loading =
        Signal::derive(move || new_item.pending().get() || locations.loading().get());

    view! {
        <ModalCreateWithModeCancel
            title="New Item"
            loading=global_loading
            on_creation=move |mode| new_item.dispatch(mode)
            on_cancel=move || on_action(None)
        >
            <div class="form-control">
                <label>
                    <span class="label">"Item Name"</span>
                    <input
                        prop:value=move || name.get()
                        on:input=move |ev| name.set(event_target_value(&ev))
                        type="text"
                        class="input input-bordered w-full"
                        placeholder="e.g. Pizza"
                        required=true
                        maxlength=60
                    />
                </label>
            </div>
            <div class="form-control">
                <label>
                    <span class="label">"Item Location"</span>
                    <select
                        on:change=move |ev| location_id.set(event_target_value(&ev))
                        class="select select-bordered w-full"
                        required=true
                    >
                        <option value="">"__Pick A Location__"</option>
                        {move || {
                            locations.get().unwrap_or_default().into_iter().map(|location| {
                                view!{ <option value=location.id>{location.name}</option>}
                            }).collect_view()
                        }}
                    </select>
                </label>
            </div>
        </ModalCreateWithModeCancel>
    }
}
