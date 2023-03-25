use crate::contexts::prelude::{use_api, use_login, CurrentApi, CurrentLogin};
use leptos::*;
use leptos_router::use_params_map;
use mcc_frontend_types::{recipe::Recipe, Fraction};

#[component]
fn RecipePrintContent(cx: Scope, recipe: Recipe) -> impl IntoView {
    let CurrentLogin { login, .. } = use_login(cx);
    let media_url = move || login.get().expect("expected login to exist").media_url;

    view! {cx,
        <>
            {
                if let Some(image_id) = recipe.image_id.as_ref() {
                    view!{cx,
                        <>
                            <figure class="h-64 w-full mb-4">
                                <img
                                    class="object-cover w-full h-full rounded"
                                    src={format!("{}/recipe-image/{}", media_url(), image_id)}
                                />
                            </figure>
                        </>
                    }
                } else {
                    view!{cx, <></>}
                }
            }
            <h1 class="text-3xl font-bold mb-4">{recipe.title}</h1>
            <table class="table table-compact table-zebra mb-4">
                <tbody>
                    {
                        if let Some(v) = &recipe.info.yields {
                            view!{cx,
                                <tr>
                                    <th class="pl-0">{v.unit_type.clone()}</th>
                                    <td>{v.value.to_string()}</td>
                                </tr>
                            }
                        } else {
                            view!{cx,
                                <tr>
                                    <th class="pl-0">"Servings"</th>
                                    <td>"0"</td>
                                </tr>
                            }
                        }
                    }
                </tbody>
            </table>
            <div class="mb-4">
                <h2 class="text-xl font-bold mb-1">"Description"</h2>
                <p>{recipe.short_description}</p>
            </div>
            <div class="mb-4">
                <h2 class="text-xl font-bold mb-1">"Notes"</h2>
                <pre class="whitespace-normal text-base font-sans">{recipe.long_description}</pre>
            </div>
            <div class="mb-4">
                <h2 class="text-xl font-bold mb-1">{"Ingredients"}</h2>
                <table class="table table-compact table-zebra w-full">
                    <thead>
                        <tr>
                            <th>"Amount"</th>
                            <th>"Name"</th>
                            <th>"Notes"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            recipe.ingredients.iter().map(|ingredient| {
                                view!{cx,
                                    <tr>
                                        <td class="whitespace-normal">{format!("{} {}", Fraction::from(ingredient.amount), {&ingredient.unit_type})}</td>
                                        <td class="whitespace-normal">{&ingredient.name}</td>
                                        <td class="whitespace-normal">{&ingredient.description.clone().unwrap_or_default()}</td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
            </div>
            <div class="mb-4">
                <h2 class="text-xl font-bold mb-1">"Steps"</h2>
                <ul>
                {
                    recipe.steps.iter().enumerate().map(|(i, step)| {
                        view!{cx,
                            <li class="mb-2">
                                <h2 class="text-l font-bold mb-2">{&step.title.clone().unwrap_or_else(|| format!("Step {}", i+1))}</h2>
                                <pre class="whitespace-normal text-base font-sans">{&step.description}</pre>
                            </li>
                        }
                    }).collect::<Vec<_>>()
                }
                </ul>
            </div>
        </>
    }
}

#[component]
pub fn RecipePrint(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = move || params.with(|params| params.get("id").cloned());
    let CurrentApi { api, .. } = use_api(cx);

    let recipe = create_resource(
        cx,
        || {},
        move |_| async move {
            let api = api.get().expect("api expected to exist");
            let id = id().expect("id expected to exist");
            match api.get_recipe_by_id(id).await {
                Ok(recipe) => Some(recipe),
                Err(_) => None,
            }
        },
    );

    view! {cx,
        <div class="p-2" data-theme="light">
            {move || {
                if let Some(recipe) = recipe.read(cx) {
                    if let Some(recipe) = recipe {
                        view!{cx, <><RecipePrintContent recipe=recipe/></>}
                    } else {
                        view!{cx, <><div>"Failed To Load :("</div></>}
                    }
                } else {
                    view!{cx, <><div>"Loading..."</div></>}
                }
            }}
        </div>
    }
}