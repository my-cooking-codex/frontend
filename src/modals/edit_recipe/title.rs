use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::{ModalSaveCancel, ModalSaveCancelProps},
};
use leptos::*;
use mcc_frontend_types::recipe::UpdateRecipe;

#[component]
pub fn EditTitleModal<F>(cx: Scope, id: String, title: String, on_action: F) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let title = create_rw_signal(cx, title);

    let update_title = create_action(cx, move |_: &()| {
        let id = id.clone();
        let title = title.get();
        let api = api.get().expect("api expected to be set");
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        title: Some(title.clone()),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(title)),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "saving recipe title"));
                }
            }
        }
    });

    view! {cx,
        <ModalSaveCancel
            title="Edit Title".to_owned()
            loading=update_title.pending()
            on_save=move || update_title.dispatch(())
            on_cancel=move || on_action(None)
        >
            <div class="form-control">
                <span class="label">"Update Title"</span>
                <label class="input-group">
                    <span>"Title"</span>
                    <input
                        prop:value=move || title.get()
                        on:input=move |ev| {title.set(event_target_value(&ev))}
                        type="text"
                        class="input input-bordered w-full"
                        placeholder="e.g. Pizza"
                        maxlength=60
                        required=true
                    />
                </label>
            </div>
        </ModalSaveCancel>
    }
}
