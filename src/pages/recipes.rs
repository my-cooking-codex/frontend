use std::collections::HashSet;

use crate::{
    components::{
        collapse::*,
        drawer::*,
        image_links::*,
        input::{ThreeStateSelect, ThreeStateSelectProps},
    },
    contexts::prelude::{use_api, use_login, use_toasts, CurrentApi, CurrentLogin},
    helpers::{api_error_to_toast, login_redirect_effect, logout_on_401, LoginState},
};
use leptos::ev::SubmitEvent;
use leptos::*;
use mcc_frontend_types::query::RecipesFilter;

#[component]
fn LabelsSelector<F>(cx: Scope, labels: HashSet<String>, on_change: F) -> impl IntoView
where
    F: Fn(HashSet<String>) + 'static + Copy,
{
    let (labels, ..) = create_signal(cx, labels);
    let selected_labels = create_rw_signal(cx, HashSet::<String>::new());
    let label_input = create_rw_signal(cx, String::new());

    let on_add_label = move |new_label: String| {
        selected_labels.update(|labels| {
            labels.insert(new_label);
        });
        on_change(selected_labels.get());
        label_input.set(String::new());
    };

    let on_remove_label = move |label| {
        selected_labels.update(|labels| {
            labels.remove(&label);
        });
        on_change(selected_labels.get());
    };

    view! {cx,
            <div class="w-80">
                <div class="flex column gap-2 mb-2">
                    <input
                        on:input=move |ev| label_input.set(event_target_value(&ev))
                        prop:value=move || label_input.get()
                        on:keydown=move |ev| {
                            if ev.key_code() == 13 {
                                ev.prevent_default();
                                let label = label_input.get();
                                if !label.is_empty() {
                                    on_add_label(label_input.get());
                                }
                            }
                        }
                        type="text"
                        class="input input-bordered input-sm w-full"
                        placeholder="Search Label..."
                        list="labels"
                    />
                    <datalist id="labels">
                        {move || {
                            let labels = labels.get();
                            labels.difference(&selected_labels.get()).map(|label| {
                                view! {cx, <option value=label/>}
                            }).collect::<Vec<_>>()
                        }}
                    </datalist>
                    <button
                        on:click=move |_| {
                            let label = label_input.get();
                            if !label.is_empty() {
                                on_add_label(label);
                            }
                        }
                        type="button"
                        class="btn btn-sm"
                    >
                        "Add"
                    </button>
                </div>
                <div class="flex flex-wrap gap-2 max-h-28 overflow-y-auto">
                    {move || {
                        let labels = selected_labels.get();
                        labels.into_iter().map(|label| view! {cx,
                            <div
                                class="inline-flex items-center bg-info text-info-content p-1 gap-2 rounded-lg"
                            >
                                {&label}
                                <button
                                    on:click=move |_| {on_remove_label(label.clone())}
                                    type="button"
                                    class="btn btn-sm"
                                >
                                    "X"
                                </button>
                            </div>
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
    }
}

#[component]
fn RecipesFilterPanel<F>(cx: Scope, filters: RecipesFilter, update_filters: F) -> impl IntoView
where
    F: Fn(RecipesFilter) + 'static,
{
    let CurrentApi { api, .. } = use_api(cx);
    let labels = create_resource(
        cx,
        || {},
        move |()| async move {
            let api = api.get().expect("api expected to exist");
            match api.get_labels().await {
                Ok(v) => v,
                Err(_) => {
                    vec![]
                }
            }
        },
    );
    let filters = create_rw_signal(cx, filters);

    let on_search_submission = move |ev: SubmitEvent| {
        ev.prevent_default();
        update_filters(filters.get());
    };

    view! {cx,
        <form on:submit=on_search_submission class="flex flex-col gap-2">
            <input
                on:input=move |ev| filters.update(|filters| {
                    let v = event_target_value(&ev);
                    filters.title = if v.is_empty() { None } else { Some(v) };
                })
                prop:value=move || filters.get().title.unwrap_or_default()
                type="text"
                class="input input-bordered input-sm w-full"
                placeholder="Recipe Title..."
                aria-label="title filter"
            />
            <CollapsableBox title="Advanced" class="bg-base-100">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Freezable"</span>
                        <ThreeStateSelect
                            value=filters.get().freezable
                            on_input=move |v| filters.update(|filters| filters.freezable = v)
                            class="select select-bordered select-sm"
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Microwave Only"</span>
                        <ThreeStateSelect
                            value=filters.get().microwave_only
                            on_input=move |v| filters.update(|filters| filters.microwave_only = v)
                            class="select select-bordered select-sm"
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Labels"</span>
                        {move || {
                            view!{cx, <LabelsSelector
                                labels=labels.read(cx).unwrap_or_default().into_iter().collect()
                                on_change=move |labels| filters.update(|filters| filters.labels = Some(labels))
                            />}
                        }}
                    </label>
                </div>
            </CollapsableBox>
            <button
                type="submit"
                class="btn btn-sm btn-wide mx-auto"
            >
                "Search"
            </button>
        </form>
    }
}

#[component]
pub fn Recipes(cx: Scope) -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home", false),
        DrawerLink::new("/recipes/new", "New Recipe", false),
        DrawerLink::new("/recipes", "Recipes", true),
    ];

    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let CurrentLogin { login, set_login } = use_login(cx);

    let filters = create_rw_signal(cx, RecipesFilter::default());
    let (items, set_items) = create_signal::<Vec<ImageLinkItem>>(cx, Vec::default());

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    let fetch_recipes = create_resource(
        cx,
        move || filters.get(),
        move |filters| {
            let api = api.get().expect("api expected to exist");
            async move {
                match api.get_recipes(&filters).await {
                    Ok(v) => Some(v),
                    Err(err) => {
                        toasts.push(api_error_to_toast(
                            &err,
                            &format!("loading recipes page {}", &filters.page),
                        ));
                        logout_on_401(&set_login, &err);
                        None
                    }
                }
            }
        },
    );

    // update the items when recipes are fetched
    create_effect(cx, move |_| {
        let media_url = login.get().expect("expected login to exist").media_url;
        if let Some(Some(recipes)) = fetch_recipes.read(cx) {
            set_items.update(|v| {
                if filters.get().page == 1 {
                    v.clear();
                }
                v.extend(recipes.iter().map(|recipe| {
                    ImageLinkItem {
                        key: recipe.id.clone(),
                        href: format!("/recipes/{}", recipe.id),
                        title: recipe.title.clone(),
                        image_src: recipe
                            .image_id
                            .as_ref()
                            .map(|v| format!("{}/recipe-image/{}", media_url, v)),
                    }
                }));
            });
        }
    });

    let on_new_filters = move |new_filters: RecipesFilter| {
        filters.update(|v| {
            *v = new_filters;
            v.page = 1;
        });
    };

    let on_load_more_click = move |_| {
        filters.update(|v| {
            v.page += 1;
        });
    };

    let on_retry_click = move |_| {
        fetch_recipes.refetch();
    };

    view! {cx,
        <Drawer links={drawer_links}>
            <div class="p-4 mb-2 rounded bg-base-200">
                <h1 class="text-3xl font-bold mb-4">"Recipes"</h1>
                <RecipesFilterPanel filters=filters.get() update_filters=on_new_filters />
            </div>
            <div class="p-4 rounded bg-base-200">
                <ImageLinksBox items={items} />
                {move || {
                    match (fetch_recipes.loading().get(), fetch_recipes.read(cx)) {
                        // it's loading
                        (true, _) => view! {cx,
                            <><button
                                type="button"
                                class="btn btn-block loading">
                                "Loading..."
                            </button></>
                        },
                        (false, Some(recipes)) => recipes.map(|recipes| {
                            // if we got the max number of recipes,
                            // we can assume there are more
                            if recipes.len() == filters.get().per_page {
                                view! {cx,
                                    <><button
                                        type="button"
                                        class="btn btn-block"
                                        on:click={on_load_more_click}>
                                        "More"
                                    </button></>
                                }
                            } else {
                                // we got less than the max number of recipes,
                                // so we're at the bottom
                                view! {cx, <><div class="text-center">"Reached Bottom"</div></>}
                            }
                        }).unwrap_or_else(|| {
                            // some error was handled
                            view! {cx,
                                <><button
                                    type="button"
                                    class="btn btn-block"
                                    on:click={on_retry_click}>
                                    "More, (Retry)"
                                </button></>
                            }
                        }),
                        // some error was handled
                        (false, None) => view! {cx,
                            <><button
                                type="button"
                                class="btn btn-block"
                                on:click={on_retry_click}>
                                "More, (Retry)"
                            </button></>
                        },
                    }
                }}
            </div>
        </Drawer>
    }
}
