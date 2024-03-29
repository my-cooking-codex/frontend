use crate::{
    components::input::FractionalInput,
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::*,
};
use leptos::*;
use mcc_frontend_types::recipe::{Ingredient, UpdateRecipe};

enum EditIngredientEvent {
    Update(Ingredient),
    Delete,
}

#[component]
fn EditIngredient<F>(index: usize, ingredient: Ingredient, on_event: F) -> impl IntoView
where
    F: Fn(usize, EditIngredientEvent) + 'static + Copy,
{
    let ingredient = create_rw_signal(ingredient);

    create_effect(move |_| {
        on_event(index, EditIngredientEvent::Update(ingredient.get()));
    });

    view! {
        <div class="mb-4 p-4 rounded bg-base-200">
            <div class="grid grid-cols-[auto_3rem] gap-2 mb-2">
                <input
                    prop:value=move || ingredient.get().name
                    on:input=move |ev| ingredient.update(|i| i.name = event_target_value(&ev))
                    type="text"
                    class="input input-bordered w-full"
                    placeholder="e.g. Carrots"
                    required=true
                />
                <button
                    on:click=move |_| on_event(index, EditIngredientEvent::Delete)
                    type="button"
                    class="btn shadow-lg">
                    "X"
                </button>
            </div>
            <div class="grid grid-cols-[8rem_auto] gap-2 mb-2">
                <FractionalInput
                    value=ingredient.get().amount
                    on_input=move |amount| ingredient.update(|i| i.amount = amount)
                    class="input input-bordered w-full".to_owned()
                    placeholder="e.g. 15".to_owned()
                    required=true
                />
                <input
                    prop:value=move || ingredient.get().unit_type
                    on:input=move |ev| ingredient.update(|i| i.unit_type = event_target_value(&ev))
                    type="text"
                    class="input input-bordered w-full"
                    placeholder="e.g. g"
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
                prop:value=move || ingredient.get().description.unwrap_or_default()
                on:input=move |ev| ingredient.update(|i| i.description = Some(event_target_value(&ev)))
                type="text"
                class="textarea textarea-bordered w-full"
                placeholder="e.g. Diced..."
            />
        </div>
    }
}

#[component]
pub fn EditIngredientsModal<F>(
    id: String,
    ingredients: Vec<Ingredient>,
    on_action: F,
) -> impl IntoView
where
    F: Fn(Option<Vec<Ingredient>>) + 'static + Copy,
{
    let toasts = use_toasts();
    let CurrentApi { api, .. } = use_api();
    let ingredients = create_rw_signal(ingredients);

    let update_recipe = create_action(move |_: &()| {
        let id = id.clone();
        let api = api.get_untracked().expect("api expected to be set");
        let ingredients = ingredients.get_untracked();
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        ingredients: Some(ingredients.iter().map(|i| i.clone().into()).collect()),
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

    view! {
        <ModalSaveCancel
            title="Edit Ingredients"
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
                    view! {
                        <EditIngredient
                            index=i
                            ingredient=ingredient.clone()
                            on_event=on_event
                        />}
                    }).collect::<Vec<_>>()
                }}
                <button
                    on:click=move |_| ingredients.update(|ingredients| ingredients.push(Ingredient::default()))
                    type="button"
                    class="btn shadow-lg w-full">
                    "Add Ingredient"
                </button>
            </div>
        </ModalSaveCancel>
    }
}
