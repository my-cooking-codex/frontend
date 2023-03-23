use crate::{
    components::drawer::*,
    contexts::{
        login::{use_login, CurrentLogin},
        prelude::{use_api, use_toasts, CurrentApi, Toast},
    },
    helpers::{api_error_to_toast, login_redirect_effect, logout_on_401, LoginState},
};
use leptos::{ev::SubmitEvent, *};
use leptos_router::use_navigate;
use mcc_frontend_types::recipe::CreateRecipe;

#[component]
pub fn NewRecipe(cx: Scope) -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home", false),
        DrawerLink::new("/recipes/new", "New Recipe", true),
        DrawerLink::new("/recipes", "Recipes", false),
    ];

    let navigator = use_navigate(cx);
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let CurrentLogin { set_login, .. } = use_login(cx);
    let (title, set_title) = create_signal(cx, String::default());

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    let create_new_recipe = create_action(cx, move |title: &String| {
        let title = title.to_owned();
        let api = api.get().expect("api expected to exist");
        async move {
            match api
                .post_new_recipe(&CreateRecipe {
                    title,
                    ..Default::default()
                })
                .await
            {
                Ok(v) => {
                    log::debug!("new recipe created: {v:?}");
                    toasts.push(Toast {
                        message: "Recipe Created".to_owned(),
                    });
                    Some(v)
                }
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "creating recipe"));
                    logout_on_401(&set_login, &err);
                    None
                }
            }
        }
    });

    // Navigate to the new recipe page after it's created successfully
    create_effect(cx, move |_| {
        if let Some(recipe) = create_new_recipe.value().get() {
            if let Some(recipe) = recipe {
                navigator(&format!("/recipes/{}", recipe.id), Default::default()).unwrap();
            }
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        create_new_recipe.dispatch(title.get());
    };

    view! {cx,
        <Drawer links={drawer_links}>
            <div class="p-4 rounded bg-base-200">
                <h1 class={"text-3xl font-bold mb-2"}>"New Recipe"</h1>
                <form on:submit=on_submit class="max-w-xs">
                    <div class="form-control mb-6">
                        <label class="label">
                            <span class="label-text">"Recipe Title"</span>
                        </label>
                        <label class="input-group">
                            <span>"Title"</span>
                            <input
                                prop:value=move || title.get()
                                on:input=move |ev| {set_title.set(event_target_value(&ev))}
                                type="text"
                                class="input input-bordered"
                                placeholder="e.g. Pizza"
                                required=true
                            />
                        </label>
                    </div>
                    <div class="form-control">
                        {move || {
                            if create_new_recipe.pending().get() {
                                view!{cx, <button type="submit" class="btn loading" disabled=true>"Create"</button>}
                            } else {
                                view!{cx, <button type="submit" class="btn btn-primary">"Create"</button>}
                            }
                        }}
                    </div>
                </form>
            </div>
        </Drawer>
    }
}
