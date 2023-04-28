use crate::{
    components::{
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
fn RecipesFilterPanel<F>(cx: Scope, filters: RecipesFilter, update_filters: F) -> impl IntoView
where
    F: Fn(RecipesFilter) + 'static,
{
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
