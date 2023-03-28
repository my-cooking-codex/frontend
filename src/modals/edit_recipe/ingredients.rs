use crate::{
    components::input::{FractionalInput, FractionalInputProps},
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::*,
};
use leptos::*;
use mcc_frontend_types::recipe::{Ingredient, UpdateIngredient, UpdateRecipe};

enum EditIngredientEvent {
    Update(Ingredient),
    Delete,
}

#[component]
fn EditIngredient<F>(
    cx: Scope,
    index: usize,
    len: usize,
    ingredient: Ingredient,
    on_event: F,
) -> impl IntoView
where
    F: Fn(usize, EditIngredientEvent) + 'static + Copy,
{
    let ingredient = create_rw_signal(cx, ingredient);

    create_effect(cx, move |_| {
        on_event(index, EditIngredientEvent::Update(ingredient.get()));
    });

    view! {cx,
        <div class="mb-4 p-4 rounded bg-base-200">
            <div class="grid grid-cols-[auto_3rem] gap-2 mb-2">
                <input
                    prop:value=move || ingredient.get().name
                    on:input=move |ev| ingredient.update(|i| i.name = event_target_value(&ev))
                    type="text"
                    class="input input-bordered w-full"
                    placeholder="name..."
                    required=true
                />
                <button
                    on:click=move |_| on_event(index, EditIngredientEvent::Delete)
                    type="button"
                    class="btn">
                    "X"
                </button>
            </div>
            <div class="grid grid-cols-[8rem_auto] gap-2 mb-2">
                <FractionalInput
                    value=ingredient.get().amount
                    on_input=move |amount| ingredient.update(|i| i.amount = amount)
                    class="input input-bordered w-full".to_owned()
                    placeholder="amount...".to_owned()
                    required=true
                />
                <input
                    prop:value=move || ingredient.get().unit_type
                    on:input=move |ev| ingredient.update(|i| i.unit_type = event_target_value(&ev))
                    type="text"
                    class="input input-bordered w-full"
                    placeholder="unit..."
                    list="units"
                    required=true
                />
                <datalist id="units">
                    <option value="g" />
                    <option value="kg" />
                    <option value="ml" />
                    <option value="l" />
                    <option value="tsp" />
                    <option value="tbsp" />
                    <option value="cup" />
                    <option value="oz" />
                    <option value="lb" />
                    <option value="pinch" />
                    <option value="dash" />
                    <option value="slice" />
                    <option value="can" />
                    <option value="bottle" />
                    <option value="jar" />
                    <option value="head" />
                    <option value="stalk" />
                    <option value="bunch" />
                    <option value="handful" />
                </datalist>
            </div>
            <input
                prop:value=move || ingredient.get().description
                on:input=move |ev| ingredient.update(|i| i.description = Some(event_target_value(&ev)))
                type="text"
                class="textarea textarea-bordered w-full"
                placeholder="notes..."
            />
        </div>
    }
}

#[component]
pub fn EditIngredientsModal<F>(
    cx: Scope,
    id: String,
    ingredients: Vec<Ingredient>,
    on_action: F,
) -> impl IntoView
where
    F: Fn(Option<Vec<Ingredient>>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let ingredients = create_rw_signal(cx, ingredients);

    let update_recipe = create_action(cx, move |_: &()| {
        let id = id.clone();
        let api = api.get().expect("api expected to be set");
        let ingredients = ingredients.get();
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        ingredients: Some(
                            ingredients
                                .iter()
                                .map(|ingredient| UpdateIngredient {
                                    name: Some(ingredient.name.clone()),
                                    amount: Some(ingredient.amount),
                                    unit_type: Some(ingredient.unit_type.clone()),
                                    description: ingredient.description.clone(),
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(ingredients.clone())),
                Err(err) => toasts.push(api_error_to_toast(&err, "updating ingredients")),
            }
        }
    });

    let on_event = move |index: usize, ev: EditIngredientEvent| match ev {
        EditIngredientEvent::Update(ingredient) => {
            // XXX this is a bit hacky. Although it works until each step has a unique id
            ingredients.update_untracked(|ingredients| {
                ingredients[index] = ingredient;
            });
        }
        EditIngredientEvent::Delete => {
            ingredients.update(|ingredients| {
                ingredients.remove(index);
            });
        }
    };

    view! {cx,
        <ModalSaveCancel
            title="Edit Ingredients".to_owned()
            loading=update_recipe.pending()
            on_save=move || update_recipe.dispatch(())
            on_cancel=move || on_action(None)
        >
            <div class="max-h-[50vh] lg:max-h-[60vh] overflow-y-auto">
                // TODO each ingredient should have it's own unique id,
                // preventing all ingredients from being updated on a single change
                {move || {
                    let ingredients = ingredients.get();
                    ingredients.iter().enumerate().map(|(i, ingredient)| {
                    view! {cx,
                        <EditIngredient
                            index=i
                            len=ingredients.len()
                            ingredient=ingredient.clone()
                            on_event=on_event
                        />}
                    }).collect::<Vec<_>>()
                }}
                <button
                    on:click=move |_| ingredients.update(|ingredients| ingredients.push(Ingredient::default()))
                    type="button"
                    class="btn w-full">
                    "Add Ingredient"
                </button>
            </div>
        </ModalSaveCancel>
    }
}
