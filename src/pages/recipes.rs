use std::collections::HashSet;

use crate::{
    components::{
        collapse::*,
        image_links::*,
        input::{LabelSelector, ThreeStateSelect},
        loading::{BufferedPageLoader, LoadingItemsState},
    },
    contexts::prelude::{
        use_api, use_login, use_modal_controller, use_toasts, CurrentApi, CurrentLogin,
    },
    helpers::{api_error_to_toast, logout_on_401},
    modals::edit_recipe::NewRecipeModal,
};
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::use_navigate;
use mcc_frontend_types::query::RecipesFilter;

#[component]
fn RecipesFilterPanel<F>(
    #[prop(into)] filters: MaybeSignal<RecipesFilter>,
    update_filters: F,
) -> impl IntoView
where
    F: Fn(RecipesFilter) + 'static,
{
    let CurrentApi { api, .. } = use_api();
    let labels = create_resource(
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
    let filters = create_rw_signal(filters.get_untracked());

    let on_search_submission = move |ev: SubmitEvent| {
        ev.prevent_default();
        update_filters(filters.get());
    };

    view! {
        <form on:submit=on_search_submission class="flex flex-col gap-2">
            <div class="flex gap-2">
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
                <button
                    on:click=move |_| filters.update(|filters| *filters = RecipesFilter::default())
                    type="button"
                    class="btn btn-sm shadow-lg"
                >
                    "Reset"
                </button>
            </div>
            <CollapsableBox title="Advanced" class="bg-base-100">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Freezable"</span>
                        <ThreeStateSelect
                            value=Signal::derive( move || filters.get().freezable)
                            on_change=move |v| filters.update(|filters| filters.freezable = v)
                            class="select select-bordered select-sm"
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Microwave Only"</span>
                        <ThreeStateSelect
                            value=Signal::derive( move || filters.get().microwave_only)
                            on_change=move |v| filters.update(|filters| filters.microwave_only = v)
                            class="select select-bordered select-sm"
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">"Labels"</span>
                        <div class="w-80 max-h-28">
                            <LabelSelector
                                labels=Signal::derive( move || HashSet::from_iter(labels.get().unwrap_or_default().into_iter()))
                                allow_new=false
                                compact=true
                                selected=Signal::derive( move || filters.get().labels.unwrap_or_default())
                                on_change=move |l| filters.update(|f| f.labels = Some(l))
                            />
                        </div>
                    </label>
                </div>
            </CollapsableBox>
            <button
                type="submit"
                class="btn btn-sm btn-wide btn-neutral shadow-lg mx-auto"
            >
                "Search"
            </button>
        </form>
    }
}

#[component]
pub fn Recipes() -> impl IntoView {
    let toasts = use_toasts();
    let modal_controller = use_modal_controller();
    let CurrentApi { api, .. } = use_api();
    let CurrentLogin { login, set_login } = use_login();

    let filters = create_rw_signal(RecipesFilter::default());
    let (items, set_items) = create_signal::<Vec<ImageLinkItem>>(Vec::default());

    let fetch_recipes = create_resource(
        move || filters.get(),
        move |filters| {
            let api = api.get_untracked().expect("api expected to exist");
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
    create_effect(move |_| {
        // XXX this is not great, but it works (just ensure any reads everything is x.get_untracked()
        let media_url = login
            .get_untracked()
            .expect("expected login to exist")
            .media_url;
        if let Some(Some(recipes)) = fetch_recipes.get() {
            set_items.update(|v| {
                if filters.get_untracked().page == 1 {
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

    let loading_items_state = Signal::derive(move || {
        match (fetch_recipes.loading().get(), fetch_recipes.get().flatten()) {
            (true, _) => LoadingItemsState::Loading,
            (false, Some(items)) => LoadingItemsState::Loaded(items.len()),
            (false, None) => LoadingItemsState::Failed,
        }
    });

    let on_new_recipe_action = move |recipe_id: Option<String>| {
        if let Some(recipe_id) = recipe_id {
            let navigator = use_navigate();
            navigator(&format!("/recipes/{}", recipe_id), Default::default());
        }
        modal_controller.close();
    };

    let on_new_filters = move |new_filters: RecipesFilter| {
        filters.update(|v| {
            *v = new_filters;
            v.page = 1;
        });
    };

    let on_new_recipe_click = move |_| {
        modal_controller.open(
            view! {
                <NewRecipeModal
                    on_action=on_new_recipe_action
                />
            }
            .into_view(),
        )
    };

    let on_load_more = move || {
        filters.update(|v| {
            v.page += 1;
        });
    };

    let on_retry = move || {
        fetch_recipes.refetch();
    };

    view! {
        <div class="rounded bg-base-200 p-4 mb-2">
            <h1 class="text-3xl font-bold mb-4">"Recipes"</h1>
            <div class="join shadow-lg">
                <button
                    on:click=on_new_recipe_click
                    class="btn btn-neutral shadow-lg join-item"
                >"New Recipe"</button>
            </div>
        </div>
        <div class="p-4 rounded bg-base-200">
            <RecipesFilterPanel filters=filters.read_only() update_filters=on_new_filters />
            <div class="divider" />
            <ImageLinksBox items={items} />
            <BufferedPageLoader
                items_state=loading_items_state
                items_per_page=Signal::derive( move || filters.get().per_page)
                load_more_action=on_load_more
                retry_action=on_retry
            />
        </div>
    }
}
