use crate::contexts::prelude::{use_api, use_login, CurrentApi, CurrentLogin};
use leptos::*;
use leptos_router::use_params_map;
use mcc_frontend_types::{recipe::Recipe, Fraction, HourMinuteSecond};

#[component]
fn RecipePrintContent(cx: Scope, recipe: Recipe) -> impl IntoView {
    let CurrentLogin { login, .. } = use_login(cx);
    let media_url = move || login.get().expect("expected login to exist").media_url;

    view! {cx,
        <>
            {
                if let Some(image_id) = recipe.image_id.as_ref() {
                    Some(view!{cx,
                            <figure class="h-64 w-full mb-4">
                                <img
                                    class="object-cover w-full h-full rounded"
                                    src={format!("{}/recipe-image/{}", media_url(), image_id)}
                                />
                            </figure>
                    })
                } else { None }

            }
            <h1 class="text-3xl font-bold mb-4">{recipe.title}</h1>
            <div class=" mb-4">
                <table class="table table-compact table-zebra w-full max-w-2xl">
                    <tbody>
                        {
                            let info = &recipe.info;
                            view!{cx,
                                {if let Some(v) = &info.yields {
                                    view!{cx,
                                        <tr class="text-center">
                                            <th>{&v.unit_type}</th>
                                            <th>"Freezable"</th>
                                            <th>"Microwave Only"</th>
                                        </tr>
                                    }
                                } else {
                                    view!{cx,
                                        <tr class="text-center">
                                            <th>"Servings"</th>
                                            <th>"Freezable"</th>
                                            <th>"Microwave Only"</th>
                                        </tr>
                                    }
                                }}
                                <tr class="text-center">
                                    <td>{info.yields.clone().unwrap_or_default().value}</td>
                                    <td><input prop:checked=info.freezable type="checkbox" class="checkbox" disabled=true/></td>
                                    <td><input prop:checked=info.microwave_only type="checkbox" class="checkbox" disabled=true/></td>
                                </tr>
                                <tr class="text-center">
                                    <th>"Total Time"</th>
                                    <th>"Prep Time"</th>
                                    <th>"Cook Time"</th>
                                </tr>
                                <tr class="text-center">
                                    <td>{HourMinuteSecond::from_secs(info.prep_time + info.cook_time).as_hms()}</td>
                                    <td>{HourMinuteSecond::from_secs(info.prep_time).as_hms()}</td>
                                    <td>{HourMinuteSecond::from_secs(info.cook_time).as_hms()}</td>
                                </tr>
                            }
                        }
                    </tbody>
                </table>
                {
                    if let Some(source) = recipe.info.source {
                        if !source.is_empty() {
                            Some(view!{cx, <p class="text-sm my-2">"Source: " {source}</p>})
                        } else { None }
                    } else { None }
                }
            </div>
            <div class="mb-4">
                <h2 class="text-xl font-bold mb-1">"Description"</h2>
                <p>{recipe.short_description}</p>
            </div>
            <div class="mb-4">
                <h2 class="text-xl font-bold mb-1">"Notes"</h2>
                <pre class="whitespace-pre-line text-base font-sans">{recipe.long_description}</pre>
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
                                <pre class="whitespace-pre-line text-base font-sans">{&step.description}</pre>
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
            <button
                on:click=move |_| window().print().unwrap()
                class="btn btn-primary my-4 mx-auto block print:hidden"
            >
                "Click Here To Print Or: "
                <kbd data-theme="light" class="kbd">"ctrl"</kbd>
                "+"
                <kbd data-theme="light" class="kbd">"p"</kbd>
            </button>
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
