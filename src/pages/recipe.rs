use leptos::*;
use leptos_router::{use_navigate, use_params_map};

use crate::{
    components::drawer::*,
    contexts::prelude::{use_api, use_login, use_toasts, CurrentApi, CurrentLogin},
    helpers::{api_error_to_toast, login_redirect_effect, logout_on_401, LoginState},
};
use mcc_frontend_types::{recipe::Recipe, Fraction};

#[component]
fn RecipeContent(cx: Scope, recipe: Recipe) -> impl IntoView {
    let navigator = use_navigate(cx);
    let CurrentLogin { login, .. } = use_login(cx);
    let media_url = move || login.get().expect("expected login to exist").media_url;
    let recipe = create_rw_signal(cx, recipe);

    view! {cx,
        <>
            // image
            <div class="mb-4 relative h-64">
                {move || {
                    if let Some(image_id) = recipe.get().image_id.as_ref() {
                        view!{cx,
                            <><img
                                class="object-cover w-full h-full rounded"
                                src={format!("{}/recipe-image/{}", media_url(), image_id)}
                            /></>
                        }
                    } else {
                        view!{cx, <><div class="w-full h-full bg-neutral rounded"></div></>}
                    }
                }}
                <div class="flex items-center absolute bottom-0 left-0 p-2 w-full bg-[#000000cc] rounded-b">
                    <h1
                        class="mr-auto text-2xl font-bold text-slate-300 \
                            whitespace-nowrap overflow-hidden text-ellipsis">
                        {move || recipe.get().title}
                    </h1>
                    <button class="btn">"Edit"</button>
                    <button class="btn">"Edit Image"</button>
                </div>
            </div>
            // toolbar
            <div class="mb-4 p-4 rounded bg-base-200">
                <button class="btn">"Print"</button>
                <div class="dropdown dropdown-bottom">
                    <label tabindex="0" class="btn m-1">"Remove"</label>
                    <div class="dropdown-content menu bg-base-200 rounded">
                        <button
                            tabindex="0"
                            class="btn btn-outline btn-error"
                            aria-label="Confirm Deletion">
                            "Confirm"
                        </button>
                    </div>
                </div>
            </div>
            // info
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">"Info"</h2>
                    <button class="btn">"Edit"</button>
                </div>
                <table class="table">
                    <tbody>
                        {move || {
                            if let Some(v) = &recipe.get().info.yields {
                                view!{cx,
                                    <tr>
                                        <th>{&v.unit_type}</th>
                                        <td>{v.value}</td>
                                    </tr>
                                }
                            } else {
                                view!{cx,
                                    <tr>
                                        <th>"Servings"</th>
                                        <td>"-"</td>
                                    </tr>
                                }
                            }
                        }}
                    </tbody>
                </table>
            </div>
            // description (short_description)
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">"Description"</h2>
                    <button class="btn">"Edit"</button>
                </div>
                <p>{move || recipe.get().short_description}</p>
            </div>
            // notes (long_description)
            <div class="mb-4 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">"Notes"</h2>
                    <button class="btn">"Edit"</button>
                </div>
                <pre class="whitespace-normal text-base font-sans">{move || recipe.get().long_description}</pre>
            </div>
            // ingredients and steps
            <div class="flex flex-col md:flex-row gap-4">
                // ingredients
                <div class="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200">
                    <div class="flex mb-2">
                        <h2 class="text-xl font-bold mr-auto">"Ingredients"</h2>
                        <button class="btn">"Edit"</button>
                    </div>
                    <table class="table table-compact table-zebra w-full">
                        <thead>
                            <tr>
                                <th>"Amount"</th>
                                <th>"Name"</th>
                                <th>"Notes"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {move || {
                                recipe.get().ingredients.iter().map(|ingredient| {
                                    view!{cx,
                                        <tr>
                                            <td class="whitespace-normal">
                                                {format!("{} {}", Fraction::from(ingredient.amount), {&ingredient.unit_type})}
                                            </td>
                                            <td class="whitespace-normal">{&ingredient.name}</td>
                                            <td class="whitespace-normal">{&ingredient.description.to_owned().unwrap_or_default()}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </tbody>
                    </table>
                </div>
                // steps
                <div class="w-full p-4 rounded bg-base-200">
                    <div class="flex mb-2">
                        <h2 class="text-xl font-bold mr-auto">"Steps"</h2>
                        <button class="btn">"Edit"</button>
                    </div>
                    <ul>
                        {move || {
                            recipe.get().steps.iter().enumerate().map(|(i, step)| {
                                view!{cx,
                                    <li class="mb-2">
                                        <h2 class="text-l font-bold mb-2">
                                            {&step.title.to_owned().unwrap_or_else(|| format!("Step {}", i+1))}
                                        </h2>
                                        <pre class="whitespace-normal text-base font-sans">{&step.description}</pre>
                                    </li>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </ul>
                </div>
            </div>
        </>
    }
}

#[component]
pub fn RecipePage(cx: Scope) -> impl IntoView {
    let drawer_links = vec![
        DrawerLink::new("/", "Home", false),
        DrawerLink::new("/recipes/new", "New Recipe", false),
        DrawerLink::new("/recipes", "Recipes", false),
    ];

    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned());

    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let CurrentLogin { set_login, .. } = use_login(cx);

    login_redirect_effect(cx, LoginState::Authenticated, "/login");

    let recipe = create_resource(
        cx,
        || {},
        move |_| async move {
            let api = api.get().expect("api expected to exist");
            let id = id().expect("id expected to exist");
            match api.get_recipe_by_id(id).await {
                Ok(recipe) => Some(recipe),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "loading recipe"));
                    logout_on_401(&set_login, &err);
                    None
                }
            }
        },
    );

    view! { cx,
        <Drawer links={drawer_links}>
            {move || {
                if let Some(recipe) = recipe.read(cx) {
                    if let Some(recipe) = recipe {
                        view! {cx, <><RecipeContent recipe={recipe} /></>}
                    } else {
                        view! {cx, <><div>"Failed To Load :("</div></>}
                    }
                } else {
                    view! {cx, <><div>"Loading..."</div></>}
                }
            }}
        </Drawer>
    }
}
