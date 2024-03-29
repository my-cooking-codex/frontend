use std::ops::Not;

use leptos::*;
use leptos_router::{use_navigate, use_params_map};

use crate::{
    components::{collapse::*, input::DropdownConfirm},
    contexts::prelude::{
        use_api, use_login, use_modal_controller, use_toasts, CurrentApi, CurrentLogin,
    },
    helpers::{api_error_to_toast, logout_on_401},
    modals::edit_recipe::*,
};
use mcc_frontend_types::{recipe::Recipe, Fraction, HourMinuteSecond};

#[component]
fn RecipeContent(recipe: Recipe) -> impl IntoView {
    let toasts = use_toasts();
    let modal_controller = use_modal_controller();
    let CurrentApi { api, .. } = use_api();
    let CurrentLogin { login, set_login } = use_login();
    let media_url = move || login.get().expect("expected login to exist").media_url;
    let recipe = create_rw_signal(recipe);
    let edit_mode = create_rw_signal(false);

    let delete_action = create_action(move |_: &()| async move {
        let navigator = use_navigate();
        let api = api.get().expect("expected api to exist");
        match api.delete_recipe(&recipe.get().id).await {
            Ok(_) => {
                navigator("/recipes", Default::default());
                true
            }
            Err(err) => {
                toasts.push(api_error_to_toast(&err, "deleting recipe"));
                logout_on_401(&set_login, &err);
                false
            }
        }
    });

    let on_labels_edit_action = move |new_labels| {
        if let Some(new_labels) = new_labels {
            recipe.update(|r| r.labels = new_labels);
        }
        modal_controller.close();
    };

    let on_title_edit_action = move |new_title| {
        if let Some(new_title) = new_title {
            recipe.update(|r| r.title = new_title);
        }
        modal_controller.close();
    };

    let on_image_edit_action = move |new_image_id| {
        if let Some(new_image_id) = new_image_id {
            recipe.update(|r| r.image_id = new_image_id);
        }
        modal_controller.close();
    };

    let on_info_edit_action = move |new_info| {
        if let Some(new_info) = new_info {
            recipe.update(|r| r.info = new_info);
        }
        modal_controller.close();
    };

    let on_description_edit_action = move |new_description| {
        if let Some(new_description) = new_description {
            recipe.update(|r| r.short_description = Some(new_description));
        }
        modal_controller.close();
    };

    let on_long_description_edit_action = move |new_description| {
        if let Some(new_description) = new_description {
            recipe.update(|r| r.long_description = Some(new_description));
        }
        modal_controller.close();
    };

    let on_ingredients_edit_action = move |new_ingredients| {
        if let Some(new_ingredients) = new_ingredients {
            recipe.update(|r| r.ingredients = new_ingredients);
        }
        modal_controller.close();
    };

    let on_steps_edit_action = move |new_steps| {
        if let Some(new_steps) = new_steps {
            recipe.update(|r| r.steps = new_steps);
        }
        modal_controller.close();
    };

    let on_print_click = move |_| {
        let id = recipe.get().id;
        let print_window = window()
            .open_with_url_and_target(&format!("/recipes/{id}/print"), "_blank")
            .unwrap();
        if let Some(print_window) = print_window {
            print_window.open().unwrap();
        }
    };

    let on_edit_labels_click = move |_| {
        modal_controller.open(
            view! {
                    <EditLabelsModal
                        id=recipe.get().id
                        labels=recipe.get().labels
                        on_action=on_labels_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_title_click = move |_| {
        modal_controller.open(
            view! {
                    <EditTitleModal
                        id=recipe.get().id
                        title=recipe.get().title
                        on_action=on_title_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_image_click = move |_| {
        modal_controller.open(
            view! {
                    <EditImageModal
                        id=recipe.get().id
                        image_id=recipe.get().image_id
                        on_action=on_image_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_info_click = move |_| {
        modal_controller.open(
            view! {
                    <EditInfoModal
                        id=recipe.get().id
                        info=recipe.get().info
                        on_action=on_info_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_description_click = move |_| {
        modal_controller.open(
            view! {
                    <EditDescriptionModal
                        id=recipe.get().id
                        description=recipe.get().short_description.unwrap_or_default()
                        on_action=on_description_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_long_description_click = move |_| {
        modal_controller.open(
            view! {
                    <EditLongDescriptionModal
                        id=recipe.get().id
                        description=recipe.get().long_description.unwrap_or_default()
                        on_action=on_long_description_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_ingredients_click = move |_| {
        modal_controller.open(
            view! {
                    <EditIngredientsModal
                        id=recipe.get().id
                        ingredients=recipe.get().ingredients
                        on_action=on_ingredients_edit_action
                    />
            }
            .into_view(),
        );
    };

    let on_edit_steps_click = move |_| {
        modal_controller.open(
            view! {
                    <EditStepsModal
                        id=recipe.get().id
                        steps=recipe.get().steps
                        on_action=on_steps_edit_action
                    />
            }
            .into_view(),
        );
    };

    view! {
        // image
        <div class="mb-4 relative h-64">
            {move || {
                if let Some(image_id) = recipe.get().image_id.as_ref() {
                    view!{
                        <img
                            class="object-cover w-full h-full rounded-md"
                            src={format!("{}/recipe-image/{}", media_url(), image_id)}
                        />
                    }.into_any()
                } else {
                    view!{ <div class="w-full h-full bg-neutral rounded"></div>}.into_any()
                }
            }}
            <div class="flex items-center absolute bottom-0 left-0 p-2 w-full bg-base-300/[.8] backdrop-blur-sm rounded-b-md">
                <h1
                    class="mr-auto text-2xl font-bold \
                        whitespace-nowrap overflow-hidden text-ellipsis py-2">
                    {move || recipe.get().title}
                </h1>
                <Show when=move || edit_mode.get()>
                    <div class="join">
                        <button on:click=on_edit_title_click class="btn join-item">"Edit"</button>
                        <button on:click=on_edit_image_click class="btn join-item">"Edit Image"</button>
                    </div>
                </Show>
            </div>
        </div>
        // toolbar
        <div class="mb-4 flex flex-wrap gap-2">
            // general tools
            <div class="flex flex-wrap gap-2 mr-auto">
                <button on:click=on_print_click class="btn shadow-lg">"Print"</button>
                <button on:click=on_edit_labels_click class="btn shadow-lg">"Labels"</button>
            </div>
            // edit tools
            <div class="flex flex-wrap gap-2">
                <label
                    class="swap btn shadow-lg"
                    // class="btn-outline"
                    class:btn-outline=move || edit_mode.get()
                    >
                    <input
                        prop:checked=move || edit_mode.get()
                        on:click=move |_| edit_mode.update(|mode| *mode=mode.not())
                        type="checkbox"
                    />
                    <div class="swap-on">"View Mode"</div>
                    <div class="swap-off">"Edit Mode"</div>
                </label>
                <DropdownConfirm
                    title="Remove"
                    confirm_aria=""
                    on_confirm=move || delete_action.dispatch(())
                    class="shadow-lg"
                />
            </div>
        </div>
        // info
        <div class="mb-4 p-4 rounded bg-base-200">
            <div class="flex mb-2">
                <h2 class="text-xl font-bold mr-auto">"Info"</h2>
                <Show when=move || edit_mode.get()>
                    <button on:click=on_edit_info_click class="btn shadow-lg">"Edit"</button>
                </Show>
            </div>
            <table class="table table-zebra w-full max-w-5xl">
                <tbody>
                    {move || {
                        let info = &recipe.get().info;
                        view!{
                            {if let Some(v) = &info.yields {
                                view!{
                                    <tr class="text-center">
                                        <th>{&v.unit_type}</th>
                                        <th>"Freezable"</th>
                                        <th>"Microwave Only"</th>
                                    </tr>
                                }
                            } else {
                                view!{
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
                    }}
                </tbody>
            </table>
            {move || {
                if let Some(source) = recipe.get().info.source {
                    if !source.is_empty() {
                        return Some(view!{ <p class="text-sm my-2">"Source: " {source}</p>});
                    }
                }
                None
            }}
        </div>
        // description (short_description)
        <div class="mb-4 p-4 rounded bg-base-200">
            <div class="flex mb-2">
                <h2 class="text-xl font-bold mr-auto">"Description"</h2>
                <Show when=move || edit_mode.get()>
                    <button on:click=on_edit_description_click class="btn shadow-lg">"Edit"</button>
                </Show>
            </div>
            <p>{move || recipe.get().short_description}</p>
        </div>
        // notes (long_description)
        <div class="mb-4 p-4 rounded bg-base-200">
            <div class="flex mb-2">
                <h2 class="text-xl font-bold mr-auto">"Notes"</h2>
                <Show when=move || edit_mode.get()>
                    <button on:click=on_edit_long_description_click class="btn shadow-lg">"Edit"</button>
                </Show>
            </div>
            <pre class="whitespace-pre-line text-base font-sans">{move || recipe.get().long_description}</pre>
        </div>
        // ingredients and steps
        <div class="flex flex-col md:flex-row gap-4">
            // ingredients
            <div class="basis-full md:basis-3/4 lg:basis-11/12 p-4 rounded bg-base-200">
                <div class="flex mb-2">
                    <h2 class="text-xl font-bold mr-auto">"Ingredients"</h2>
                    <Show when=move || edit_mode.get()>
                        <button on:click=on_edit_ingredients_click class="btn shadow-lg">"Edit"</button>
                    </Show>
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
                                view!{
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
                    <Show when=move || edit_mode.get()>
                        <button on:click=on_edit_steps_click class="btn shadow-lg">"Edit"</button>
                    </Show>
                </div>
                <div class="flex flex-col gap-2">
                    {move || {
                        recipe.get().steps.into_iter().enumerate().map(|(i, step)| {
                            view!{
                                <CollapsableBox
                                    title={step.title.unwrap_or_else(|| format!("Step {}", i+1))}
                                    open=true
                                    class="border border-base-300 bg-base-100"
                                >
                                    <pre class="whitespace-pre-line text-base font-sans">{step.description}</pre>
                                </CollapsableBox>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RecipePage() -> impl IntoView {
    let params = use_params_map();
    let id = Signal::derive(move || params.get().get("id").cloned());

    let toasts = use_toasts();
    let CurrentApi { api, .. } = use_api();
    let CurrentLogin { set_login, .. } = use_login();

    let recipe = create_resource(
        || {},
        move |_| async move {
            let api = api.get_untracked().expect("api expected to exist");
            let id = id.get_untracked().expect("id expected to exist");
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

    view! {
        {move || {
            if let Some(recipe) = recipe.get() {
                if let Some(recipe) = recipe {
                    view! { <><RecipeContent recipe={recipe} /></>}
                } else {
                    view! { <><div>"Failed To Load :("</div></>}
                }
            } else {
                view! { <><div>"Loading..."</div></>}
            }
        }}
    }
}
