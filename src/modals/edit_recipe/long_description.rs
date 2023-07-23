use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalSaveCancel,
};
use leptos::*;
use mcc_frontend_types::recipe::UpdateRecipe;

#[component]
pub fn EditLongDescriptionModal<F>(
    cx: Scope,
    id: String,
    description: String,
    on_action: F,
) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let description = create_rw_signal(cx, description);

    let update_description = create_action(cx, move |_: &()| {
        let id = id.clone();
        let description = description.get_untracked();
        let api = api.get_untracked().expect("api expected to be set");
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        long_description: Some(description.clone()),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(description)),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "saving recipe description"));
                }
            }
        }
    });

    view! {cx,
        <ModalSaveCancel
            title="Edit Notes"
            loading=update_description.pending()
            on_save=move || update_description.dispatch(())
            on_cancel=move || on_action(None)
        >
            <div class="form-control">
                <span class="label">"Update Notes"</span>
                <label class="join join-vertical">
                    <span class="label p-3 bg-base-300 join-item">"Notes"</span>
                    <textarea
                        prop:value=move || description.get()
                        on:input=move |ev| {description.set(event_target_value(&ev))}
                        class="input input-bordered w-full h-56 p-2 join-item"
                        placeholder="e.g. It was a great recipe when paired with..."
                    />
                </label>
            </div>
        </ModalSaveCancel>
    }
}
