use chrono::{DateTime, Months, NaiveDate, Utc};
use leptos::*;
use std::collections::HashSet;

use mcc_frontend_types::{
    pantry::{Item, UpdateItem},
    SelectedUpdate,
};

use crate::{
    components::input::LabelSelector,
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalSaveCancel,
};

#[component]
pub fn EditItemModal<F>(item: Item, on_action: F) -> impl IntoView
where
    F: Fn(Option<Item>) + 'static + Copy,
{
    let toasts = use_toasts();
    let CurrentApi { api, .. } = use_api();
    let item = create_rw_signal(item);

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

    let labels = create_resource(
        || {},
        move |_| async move {
            let api = api.get_untracked().expect("api expected to be set");
            match api.get_labels().await {
                Ok(v) => HashSet::from_iter(v.into_iter()),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "loading labels"));
                    HashSet::new()
                }
            }
        },
    );

    let save = create_action(move |_: &()| {
        let api = api.get_untracked().expect("api expected to be set");
        let id = item.get_untracked().id.clone();
        let item = item.get_untracked();
        let update = SelectedUpdate {
            fields: vec![
                String::from("name"),
                String::from("location_id"),
                String::from("quantity"),
                String::from("notes"),
                String::from("expiry"),
                String::from("labels"),
            ],
            model: UpdateItem {
                name: item.name.clone(),
                location_id: item.location_id.clone(),
                quantity: item.quantity,
                notes: item.notes.clone(),
                expiry: item.expiry,
                labels: item.labels.clone(),
            },
        };
        async move {
            match api.patch_pantry_item(&id, &update).await {
                Ok(_) => on_action(Some(item)),
                Err(err) => toasts.push(api_error_to_toast(&err, "saving item")),
            }
        }
    });

    let global_loading = Signal::derive(move || {
        locations.loading().get() || labels.loading().get() || save.pending().get()
    });

    view! {
        <ModalSaveCancel
            title="Edit Item"
            loading=global_loading
            on_save=move || save.dispatch(())
            on_cancel=move || on_action(None)
        >
             <div class="form-control">
                <label>
                    <span class="label">"Name"</span>
                    <input
                        prop:value=move || item.get().name
                        on:input=move |ev| item.update(|v| v.name = event_target_value(&ev))
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
                    <span class="label">"Location"</span>
                    <select
                        on:change=move |ev| item.update(|v| v.location_id = event_target_value(&ev))
                        class="select select-bordered w-full"
                    >
                        {move || {
                            locations.get().unwrap_or_default().into_iter().map(|location| {
                                view!{ <option
                                    value=&location.id
                                    selected=move || item.get().location_id == location.id
                                >{location.name}</option>}
                            }).collect_view()
                        }}
                    </select>
                </label>
            </div>
            <div class="form-control">
                <label>
                    <span class="label">"Quantity"</span>
                    <input
                        prop:value=move || item.get().quantity
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse() {
                                item.update(|v| v.quantity = value)
                            }
                        }
                        type="number"
                        class="input input-bordered w-full"
                        required=true
                        min=1
                    />
                </label>
            </div>
            <div class="form-control">
                <label>
                    <span class="label">"Expiry"</span>
                    <input
                        prop:value=move || item.get().expiry.map(|v| v.naive_local().format("%Y-%m-%d").to_string())
                        on:input=move |ev| item.update(|v| {
                            if event_target_value(&ev).is_empty() {
                                v.expiry = None
                            } else {
                                let new_time = NaiveDate::parse_from_str(&event_target_value(&ev), "%Y-%m-%d").unwrap();
                                v.expiry = Some(DateTime::<Utc>::from_naive_utc_and_offset(new_time.and_hms_milli_opt(0,0,0,0).unwrap(), Utc));
                            }
                        })
                        type="date"
                        class="input input-bordered w-full"
                    />
                </label>
                <div class="join">
                    <button
                        on:click=move |_| {
                            item.update(|item| {
                                item.expiry = item.expiry
                                    .unwrap_or_else(Utc::now)
                                    .checked_add_months(Months::new(1))
                            })
                        }
                        type="button"
                        class="btn btn-sm join-item"
                    >+1</button>
                    <button
                        on:click=move |_| {
                            item.update(|item| {
                                item.expiry = item.expiry
                                    .unwrap_or_else(Utc::now)
                                    .checked_add_months(Months::new(3))
                            })
                        }
                        type="button"
                        class="btn btn-sm join-item"
                    >+3</button>
                    <button
                        on:click=move |_| {
                            item.update(|item| {
                                item.expiry = item.expiry
                                    .unwrap_or_else(Utc::now)
                                    .checked_add_months(Months::new(12))
                            })
                        }
                        type="button"
                        class="btn btn-sm join-item"
                    >+12</button>
                </div>
            </div>
            <div class="form-control">
                <label>
                    <span class="label">"Notes"</span>
                    <input
                        prop:value=move || item.get().notes.unwrap_or_default()
                        on:input=move |ev| item.update(|v| {
                            if event_target_value(&ev).is_empty() {
                                v.notes = None
                            } else {
                                v.notes = Some(event_target_value(&ev))
                            }
                        })
                        type="text"
                        class="input input-bordered w-full"
                        placeholder="e.g. Pizza"
                    />
                </label>
            </div>
            <div class="form-control">
                <label>
                    <span class="label">"Labels"</span>
                    <LabelSelector
                        labels=Signal::derive( move || labels.get().unwrap_or_default())
                        allow_new=true
                        selected=Signal::derive( move || HashSet::from_iter(item.get().labels.into_iter()) )
                        on_change=move |new_labels| item.update(|v| v.labels = new_labels.into_iter().collect())
                    />
                </label>
            </div>
        </ModalSaveCancel>
    }
}
