use leptos::*;
use mcc_frontend_types::{pantry::Item, query::PantryFilter, HumanDateFormats};
use std::collections::HashSet;
use web_sys::SubmitEvent;

use crate::{
    components::{
        collapse::CollapsableBox,
        input::{LabelSelector, ThreeStateSelect},
        loading::{BufferedPageLoader, LoadingItemsState},
    },
    contexts::{
        login::{use_login, CurrentLogin},
        prelude::{use_api, use_modal_controller, use_toasts, CurrentApi},
    },
    helpers::{api_error_to_toast, logout_on_401},
    modals::{
        edit_pantry::{EditItemModal, LocationsModal, NewItemModal},
        CreationMode,
    },
};

#[component]
pub fn PantryFilterPanel<F>(
    cx: Scope,
    #[prop(into)] filters: MaybeSignal<PantryFilter>,
    on_change: F,
) -> impl IntoView
where
    F: Fn(PantryFilter) + 'static,
{
    let CurrentApi { api, .. } = use_api(cx);
    let labels = create_resource(
        cx,
        || {},
        move |()| async move {
            let api = api.get_untracked().expect("api expected to exist");
            match api.get_labels().await {
                Ok(v) => v,
                Err(_) => {
                    vec![]
                }
            }
        },
    );
    let locations = create_resource(
        cx,
        || {},
        move |()| async move {
            let api = api.get_untracked().expect("api expected to exist");
            match api.get_pantry_locations().await {
                Ok(v) => v,
                Err(_) => {
                    vec![]
                }
            }
        },
    );
    let filters = create_rw_signal(cx, filters.get_untracked());

    let on_search_submission = move |ev: SubmitEvent| {
        ev.prevent_default();
        on_change(filters.get());
    };
    view! {cx,
        <form on:submit=on_search_submission class="flex flex-col gap-2">
             <div class="flex gap-2">
                <input
                    on:input=move |ev| filters.update(|filters| {
                        let v = event_target_value(&ev);
                        filters.name = if v.is_empty() { None } else { Some(v) };
                    })
                    prop:value=move || filters.get().name.unwrap_or_default()
                    type="text"
                    class="input input-bordered input-sm w-full"
                    placeholder="Item Name..."
                    aria-label="name filter"
                />
                <button
                    on:click=move |_| filters.update(|filters| *filters = PantryFilter::default())
                    type="button"
                    class="btn btn-sm shadow-lg"
                >
                    "Reset"
                </button>
            </div>
             <CollapsableBox title="Advanced" class="bg-base-100">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Expired"</span>
                        <ThreeStateSelect
                            value=Signal::derive(cx, move || filters.get().expired)
                            on_change=move |v| filters.update(|filters| filters.expired = v)
                            class="select select-bordered select-sm"
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Labels"</span>
                        <div class="w-80 max-h-28">
                            <LabelSelector
                                labels=Signal::derive(cx, move || HashSet::from_iter(labels.read(cx).unwrap_or_default().into_iter()))
                                allow_new=false
                                compact=true
                                selected=Signal::derive(cx, move || filters.get().labels.unwrap_or_default())
                                on_change=move |l| filters.update(|f| f.labels = Some(l))
                            />
                        </div>
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Location"</span>
                        <select
                            on:change=move |ev| filters.update(|f| {
                                let new_id = event_target_value(&ev);
                                if new_id.is_empty() { f.location_id = None; }
                                else { f.location_id = Some(new_id); }
                                })
                            class="select select-bordered select-sm w-80"
                        >
                            <option selected=move || filters.get().location_id.is_none() value="">"__Any__"</option>
                            <For
                                each=move || locations.read(cx).unwrap_or_default()
                                key=move |location| location.id.to_owned()
                                view=move |cx, location| {
                                    let id = location.id.to_owned();
                                    view!{cx,
                                        <option
                                            selected=move || filters.get().location_id.unwrap_or_default() == id
                                            value={&location.id}>{&location.name}
                                        </option>
                                    }
                                }
                            />
                        </select>
                    </label>
                </div>
            </CollapsableBox>
            <button
                type="submit"
                class="btn btn-sm btn-wide shadow-lg mx-auto"
            >
                "Search"
            </button>
        </form>
    }
}

#[component]
fn PantryItemRow<E, D>(cx: Scope, item: Item, edit_action: E, delete_action: D) -> impl IntoView
where
    E: Fn() + 'static,
    D: Fn() + 'static,
{
    view! {cx,
        <tr>
            <td class="flex justify-center">
                {
                    let chip_color;
                    if item.is_expired() {
                        chip_color = "bg-error";
                    } else if item.is_expired_with_offset(7) {
                        chip_color = "bg-warning";
                    } else {
                        chip_color = "bg-success";
                    }
                    view!{cx, <div class=format!("h-2 w-7 rounded-full {chip_color}")></div>}
                }
            </td>
            <td>{&item.name}</td>
            <td>
                <time datetime=item.expiry.map(|v| v.to_rfc3339()).unwrap_or_default()>
                {
                    // TODO get human date format from their settings
                    match item.expiry_to_human(&HumanDateFormats::DayMonthYear) {
                        Some(v) => v,
                        None => "-".to_owned(),
                    }
                }
                </time>
            </td>
            <td class="flex justify-end">
                <div class="join shadow-lg">
                    <button
                        on:click=move |_| edit_action()
                        class="btn join-item"
                        aria-label=format!("Show More For '{}'", &item.name)
                    >"More"</button>
                    <button
                        on:click=move |_| delete_action()
                        class="btn btn-outline btn-error join-item"
                        aria-label=format!("Delete '{}'", &item.name)
                    >"X"</button>
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn Pantry(cx: Scope) -> impl IntoView {
    let toasts = use_toasts(cx);
    let modal_controller = use_modal_controller(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let CurrentLogin { set_login, .. } = use_login(cx);
    let filters = create_rw_signal(cx, PantryFilter::default());
    let items = create_rw_signal::<Vec<Item>>(cx, Vec::default());
    let new_items = create_rw_signal::<Vec<Item>>(cx, Vec::default());

    let current_page = create_resource(
        cx,
        move || filters.get(),
        move |filters| {
            let api = api.get_untracked().expect("api expected to exist");
            async move {
                match api.get_pantry_items(&filters).await {
                    Ok(v) => Some(v),
                    Err(err) => {
                        toasts.push(api_error_to_toast(
                            &err,
                            &format!("loading pantry items page {}", &filters.page),
                        ));
                        logout_on_401(&set_login, &err);
                        None
                    }
                }
            }
        },
    );

    create_effect(cx, move |_| {
        // XXX this is not great, but it works (just ensure any reads everything is x.get_untracked()
        if let Some(current_page) = current_page.read(cx).flatten() {
            new_items.set(Vec::default());
            items.update(|v| {
                if filters.get_untracked().page == 1 {
                    v.clear()
                }
                v.extend(current_page);
            });
        }
    });

    let loading_items_state = Signal::derive(cx, move || {
        match (
            current_page.loading().get(),
            current_page.read(cx).flatten(),
        ) {
            (true, _) => LoadingItemsState::Loading,
            (false, Some(items)) => LoadingItemsState::Loaded(items.len()),
            (false, None) => LoadingItemsState::Failed,
        }
    });

    let delete_item = create_action(cx, move |id: &String| {
        let api = api.get_untracked().expect("api expected to exist");
        let id = id.clone();
        async move {
            match api.delete_pantry_item_by_id(&id).await {
                Ok(_) => items.update(|items| {
                    if let Some(i) = items.iter().position(|v| v.id == id) {
                        items.remove(i);
                    }
                }),
                Err(err) => toasts.push(api_error_to_toast(&err, "deleting pantry item")),
            }
        }
    });

    let delete_new_item = create_action(cx, move |id: &String| {
        let api = api.get_untracked().expect("api expected to exist");
        let id = id.clone();
        async move {
            match api.delete_pantry_item_by_id(&id).await {
                Ok(_) => new_items.update(|items| {
                    if let Some(i) = items.iter().position(|v| v.id == id) {
                        items.remove(i);
                    }
                }),
                Err(err) => toasts.push(api_error_to_toast(&err, "deleting pantry item")),
            }
        }
    });

    let on_edit_item_action = move |item: Option<Item>| {
        if let Some(updated_item) = item {
            items.update(|items| {
                for item in items.into_iter() {
                    if item.id == updated_item.id {
                        let _ = std::mem::replace(item, updated_item);
                        break;
                    }
                }
            });
        }
        modal_controller.close();
    };

    let on_edit_new_item_action = move |item: Option<Item>| {
        if let Some(updated_item) = item {
            new_items.update(|items| {
                for item in items.into_iter() {
                    if item.id == updated_item.id {
                        let _ = std::mem::replace(item, updated_item);
                        break;
                    }
                }
            });
        }
        modal_controller.close();
    };

    let on_new_item_action = move |creation: Option<(CreationMode, Item)>| {
        if let Some((mode, new_item)) = creation {
            new_items.update(|v| {
                v.push(new_item.clone());
            });
            match mode {
                CreationMode::CreateAndEdit => modal_controller.open(view! {cx,
                    <EditItemModal
                        item=new_item
                        on_action=on_edit_item_action
                    />
                }),
                CreationMode::Create => modal_controller.close(),
            };
        } else {
            modal_controller.close();
        }
    };

    let on_locations_action = move || {
        modal_controller.close();
    };

    let on_new_item_click = move |_| {
        modal_controller.open(
            view! {cx,
                <NewItemModal
                    on_action=on_new_item_action
                />
            }
            .into_view(cx),
        );
    };

    let on_locations_click = move |_| {
        modal_controller.open(
            view! {cx,
                <LocationsModal
                    on_action=on_locations_action
                />
            }
            .into_view(cx),
        );
    };

    let on_edit_item_click = move |item: Item| {
        modal_controller.open(view! {cx,
            <EditItemModal
                item=item
                on_action=on_edit_item_action
            />
        });
    };

    let on_edit_new_item_click = move |item: Item| {
        modal_controller.open(view! {cx,
            <EditItemModal
                item=item
                on_action=on_edit_new_item_action
            />
        });
    };

    let on_new_filters = move |new_filters| {
        filters.update(|v| {
            *v = new_filters;
            v.page = 1;
        });
    };

    let on_load_more = move || {
        filters.update(|v| {
            v.page += 1;
        });
    };

    let on_retry = move || {
        current_page.refetch();
    };

    view! {cx,
        <div class="rounded bg-base-200 p-4 mb-2">
            <h1 class="text-3xl font-bold mb-4">"Pantry"</h1>
            <div class="join shadow-lg">
                <button on:click=on_new_item_click class="btn join-item btn-neutral">"New Item"</button>
                <button on:click=on_locations_click class="btn join-item btn-neutral">"Locations"</button>
            </div>
        </div>
        <Show when=move || !new_items.get().is_empty() fallback=move |_| view!{cx, <></>}>
            <div class="rounded bg-base-200 p-4 mb-2">
                <h2 class="text-2xl font-bold mb-4">"New Items"</h2>
                <div class="rounded bg-base-100 p-4">
                    <table class="table">
                        <thead>
                            <tr>
                                <th></th>
                                <th>"Name"</th>
                                <th>"Expiry"</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                        {move || {
                            // NOTE "For" component not used as it will not re-render on item edit
                            new_items.get().into_iter().map(|item|{
                                view!{cx,
                                    <PantryItemRow
                                        item=item.clone()
                                        edit_action={
                                            let item = item.clone();
                                            move || on_edit_new_item_click(item.clone())
                                        }
                                        delete_action=move || delete_new_item.dispatch(item.id.clone())
                                    />
                                }
                            }).collect_view(cx)
                        }}
                        </tbody>
                    </table>
                </div>
            </div>
        </Show>
        <div class="rounded bg-base-200 p-4">
            <PantryFilterPanel filters=filters on_change=on_new_filters />
            <div class="divider" />
            <div class="rounded bg-base-100 p-4 mb-4">
                <table class="table">
                    <thead>
                        <tr>
                            <th></th>
                            <th>"Name"</th>
                            <th>"Expiry"</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                    {move || {
                        // NOTE "For" component not used as it will not re-render on item edit
                        items.get().into_iter().map(|item|{
                            view!{cx,
                                <PantryItemRow
                                    item=item.clone()
                                    edit_action={
                                        let item = item.clone();
                                        move || on_edit_item_click(item.clone())
                                    }
                                    delete_action=move || delete_item.dispatch(item.id.clone())
                                />
                            }
                        }).collect_view(cx)
                    }}
                    </tbody>
                </table>
                <BufferedPageLoader
                    items_state=loading_items_state
                    items_per_page=Signal::derive(cx, move || filters.get().per_page)
                    load_more_action=on_load_more
                    retry_action=on_retry
                />
            </div>
        </div>
    }
}
