use std::collections::HashMap;

use leptos::*;
use mcc_frontend_types::{
    pantry::{CreateLocation, Location, UpdateLocation},
    SelectedUpdate,
};
use web_sys::SubmitEvent;

use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalNeutral,
};

#[component]
fn LocationRow<D, E>(
    cx: Scope,
    location: Location,
    loading: Signal<bool>,
    delete_callback: D,
    edit_callback: E,
) -> impl IntoView
where
    D: Fn(String) + 'static + Copy,
    E: Fn(Location) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let editing = create_rw_signal(cx, false);
    let location = create_rw_signal(cx, location);

    let delete = create_action(cx, move |_: &()| {
        let api = api.get_untracked().expect("api expected to exist");
        let id = location.get_untracked().id.clone();
        async move {
            match api.delete_pantry_location_by_id(&id).await {
                Ok(_) => delete_callback(id),
                Err(err) => toasts.push(api_error_to_toast(&err, "deleting pantry location")),
            }
        }
    });

    let save_edit = create_action(
        cx,
        move |update: &(String, SelectedUpdate<UpdateLocation>)| {
            let api = api.get_untracked().expect("api expected to exist");
            let update = update.clone();
            async move {
                match api.patch_pantry_location(&update.0, &update.1).await {
                    Ok(_) => edit_callback(location.get_untracked()),
                    Err(err) => {
                        toasts.push(api_error_to_toast(&err, "updating pantry location"));
                        // restore editing state (we want to indicate it did not save)
                        editing.set(true);
                    }
                }
            }
        },
    );

    let toggle_editing = move |want_edit: bool| {
        if !want_edit && location.get_untracked().name.is_empty() {
            // don't allow save if fields are invalid
            editing.set(editing.get_untracked());
            return;
        } else if !want_edit {
            let location = location.get_untracked();
            save_edit.dispatch((
                location.id,
                SelectedUpdate {
                    fields: vec!["name".to_owned()],
                    model: UpdateLocation {
                        name: location.name,
                    },
                },
            ));
        }
        editing.set(want_edit);
    };

    let global_loading = Signal::derive(cx, move || {
        loading.get() || save_edit.pending().get() || delete.pending().get()
    });

    view! {cx,
        <tr>
            <td>
                <Show
                    when=move || editing.get()
                    fallback=move |cx| view! {cx, <span>{&location.get().name}</span>}
                >
                    <input
                        on:input=move |ev| location.update(|v| v.name = event_target_value(&ev))
                        prop:value=move || location.get().name
                        class="input input-bordered"
                        // class="input-error"
                        class:input-error=move || location.get().name.is_empty()
                        type="text"
                        placeholder="Name..."
                        maxlength=60
                        required
                    />
                </Show>
            </td>
            <td class="flex justify-end">
                <div class="join shadow-lg">
                    <label
                        class="btn swap join-item"
                        // class="btn-disabled btn-primary loading"
                        class:btn-disabled=move || global_loading.get() || location.get().name.is_empty()
                        class:btn-primary=move || editing.get()
                        class:loading=move || save_edit.pending().get()
                    >
                        <input
                            prop:checked=move || editing.get()
                            on:input=move |ev| toggle_editing(event_target_checked(&ev))
                            type="checkbox"
                            aria-label=move || format!("Edit {}", &location.get().name)
                        />
                        <div class="swap-off">Edit</div>
                        <div class="swap-on">Save</div>
                    </label>
                    <button
                        on:click=move |_| delete.dispatch(())
                        class="btn btn-outline btn-error join-item"
                        // class="btn-disabled loading"
                        class:btn-disabled=move || global_loading.get() || editing.get()
                        class:loading=move || delete.pending().get()
                        aria-label=move || format!("Delete '{}'", &location.get().name)
                    >"X"</button>
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn LocationsModal<F>(cx: Scope, on_action: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let new_location = create_rw_signal(cx, CreateLocation::default());

    let locations = create_resource(
        cx,
        || {},
        move |()| async move {
            let api = api.get_untracked().expect("api expected to exist");
            match api.get_pantry_locations().await {
                Ok(v) => HashMap::from_iter(v.into_iter().map(|v| (v.id.to_owned(), v))),
                Err(_) => HashMap::new(),
            }
        },
    );

    let create = create_action(cx, move |new_location: &CreateLocation| {
        let api = api.get_untracked().expect("api expected to exist");
        let new_location = new_location.clone();
        async move {
            match api.post_pantry_location(&new_location).await {
                Ok(location) => locations.update(|v| {
                    v.as_mut()
                        .map(|v| v.insert(location.id.to_owned(), location));
                }),
                Err(err) => toasts.push(api_error_to_toast(&err, "creating pantry location")),
            }
        }
    });

    let edit_callback = move |new_location: Location| {
        locations.update(|v| {
            v.as_mut()
                .map(|v| v.insert(new_location.id.clone(), new_location));
        })
    };

    let delete_callback = move |id: String| {
        locations.update(|v| {
            v.as_mut().map(|v| v.remove(&id));
        })
    };

    let global_loading = Signal::derive(cx, move || {
        locations.loading().get() || create.pending().get()
    });

    let on_new_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !new_location.get_untracked().name.is_empty() {
            create.dispatch(new_location.get_untracked());
            new_location.set(CreateLocation::default());
        }
    };

    view! {cx,
        <ModalNeutral
            title="Item Locations"
            close_text="Close"
            loading=global_loading
            on_close=on_action
        >
            <div class="flex flex-col gap-4">
                <div class="bg-base-200 rounded p-4">
                    <h2 class="mb-2">"New Item"</h2>
                    <form on:submit=on_new_submit class="join shadow-lg w-full">
                        <input
                            prop:value=move || new_location.get().name
                            on:input=move |ev| new_location.update(|v| v.name = event_target_value(&ev))
                            class="input input-bordered input-sm w-full join-item"
                            type="text"
                            placeholder="Name..."
                            maxlength=60
                            required
                        />
                        <button
                            class="btn btn-sm btn-neutral join-item"
                            class:btn-disabled=move || global_loading.get()
                            type="submit"
                        >"Add"
                        </button>
                    </form>
                </div>
                <div class="bg-base-200 rounded p-4">
                    <h2 class="mb-2">"Items"</h2>
                    <table class="table bg-base-100">
                        <thead>
                            <tr>
                                <th>"Name"</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                        <For
                            each=move || locations.read(cx).unwrap_or_default().into_values()
                            key=move |v| v.id.to_owned()
                            view=move |cx, location: Location| {
                                view!{cx, <LocationRow
                                    location=location.clone()
                                    loading=global_loading
                                    delete_callback=delete_callback
                                    edit_callback=edit_callback
                                />}
                            }
                        />
                        </tbody>
                    </table>
                </div>
            </div>
        </ModalNeutral>
    }
}
