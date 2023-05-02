use leptos::*;
use mcc_frontend_types::recipe::UpdateRecipe;
use std::collections::{hash_map::RandomState, HashSet};

use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::{ModalSaveCancel, ModalSaveCancelProps},
};

#[component]
pub fn EditLabelsModal<F>(cx: Scope, id: String, labels: Vec<String>, on_action: F) -> impl IntoView
where
    F: Fn(Option<Vec<String>>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);

    let existing_labels = create_resource(
        cx,
        || {},
        move |_| async move {
            let api = api.get().expect("api expected to be set");
            match api.get_labels().await {
                Ok(labels) => HashSet::from_iter(labels.into_iter()),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "loading labels"));
                    HashSet::new()
                }
            }
        },
    );
    let labels: leptos::RwSignal<HashSet<std::string::String, RandomState>> =
        create_rw_signal(cx, HashSet::from_iter(labels.into_iter()));
    let new_label_input = create_rw_signal(cx, String::new());

    let update_labels = create_action(cx, move |_: &()| {
        let id = id.clone();
        let api = api.get().expect("api expected to be set");
        let labels = labels.get().into_iter().collect::<Vec<String>>();
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        labels: Some(labels.clone()),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(labels)),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "saving recipe labels"));
                }
            }
        }
    });

    let remove_label = move |label_name: String| {
        labels.update(|labels| {
            labels.remove(&label_name);
        });
    };

    let on_add_click = move || {
        let new_label = new_label_input.get();
        if !new_label.is_empty() {
            labels.update(|labels| {
                labels.insert(new_label.clone());
            });
            new_label_input.set(String::new());
        }
    };

    view! {cx,
        <ModalSaveCancel
            title="Edit Labels".to_owned()
            loading=update_labels.pending()
            on_save=move || update_labels.dispatch(())
            on_cancel=move || on_action(None)
        >
            <>
            <div class="my-4 flex flex-wrap gap-2">
                {move || {
                    let labels = labels.get();
                    labels.into_iter().map(|label| view! {cx,
                        <div
                            class="inline-flex items-center bg-info text-info-content p-1 gap-2 rounded-lg"
                        >
                            {&label}
                            <button
                                on:click=move |_| {remove_label(label.clone())}
                                class="btn btn-sm"
                            >
                                "X"
                            </button>
                        </div>
                    }).collect::<Vec<_>>()
                }}
            </div>
            <div class="form-control">
                <div class="input-group">
                    <input
                        prop:value=move || new_label_input.get()
                        on:input=move |ev| {new_label_input.set(event_target_value(&ev))}
                        on:keydown=move|ev| {
                            if ev.key_code() == 13 {
                                ev.prevent_default();
                                on_add_click();
                            } else {}
                        }
                        class="input input-bordered w-full"
                        type="text"
                        placeholder="e.g. High Protein..."
                        list="labels"
                        maxlength="60"
                    />
                    <datalist id="labels">
                        {move || {
                            let labels = labels.get();
                            let existing_labels = existing_labels.read(cx).unwrap_or_default();
                            existing_labels.difference(&labels).into_iter().map(|label|
                                view! {cx,<option value=label />}
                            ).collect::<Vec<_>>()
                        }}
                    </datalist>
                    <button
                        on:click=move |_| on_add_click()
                        class="btn"
                        type="button"
                    >
                        "Add"
                    </button>
                </div>
            </div>
            </>
        </ModalSaveCancel>
    }
}
