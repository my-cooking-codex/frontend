use leptos::*;
use mcc_frontend_types::recipe::CreateRecipe;

use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalCreateCancel,
};

#[component]
pub fn NewRecipeModal<F>(cx: Scope, on_action: F) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let title = create_rw_signal(cx, String::default());

    let new_recipe = create_action(cx, move |_: &()| {
        let api = api.get().expect("api expected to be set");
        let title = title.get();
        async move {
            match api
                .post_new_recipe(&CreateRecipe {
                    title,
                    ..Default::default()
                })
                .await
            {
                Ok(v) => on_action(Some(v.id)),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "creating new recipe"));
                }
            }
        }
    });

    view! {cx,
        <ModalCreateCancel
            title="New Recipe"
            loading=new_recipe.pending()
            on_creation=move || new_recipe.dispatch(())
            on_cancel=move || on_action(None)
        >
            <div class="form-control">
                <span class="label">"Create Recipe From Scratch"</span>
                <label class="join">
                    <span class="label p-3 bg-base-300 join-item">"Title"</span>
                    <input
                        prop:value=move || title.get()
                        on:input=move |ev| title.set(event_target_value(&ev))
                        type="text"
                        class="input input-bordered w-full join-item"
                        placeholder="e.g. Pizza"
                        required=true
                        maxlength=60
                    />
                </label>
            </div>
        </ModalCreateCancel>
    }
}
