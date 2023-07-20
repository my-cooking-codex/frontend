use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalSaveCancel,
};
use leptos::ev::Event;
use leptos::*;

use wasm_bindgen::JsCast;
use web_sys::{File, HtmlInputElement};

#[component]
pub fn EditImageModal<F>(
    cx: Scope,
    id: String,
    image_id: Option<String>,
    on_action: F,
) -> impl IntoView
where
    F: Fn(Option<Option<String>>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let image_id = create_rw_signal(cx, image_id);
    let image_file = create_rw_signal::<Option<File>>(cx, Option::default());

    let save_action = create_action(cx, move |_: &()| {
        let id = id.clone();
        let api = api.get().expect("api expected to be set");
        async move {
            match image_file.get() {
                Some(image_file) => match api.post_recipe_image(id, image_file).await {
                    Ok(v) => on_action(Some(Some(v))),
                    Err(err) => {
                        toasts.push(api_error_to_toast(&err, "setting new recipe image"));
                    }
                },
                None => match api.delete_recipe_image(id).await {
                    Ok(_) => on_action(Some(None)),
                    Err(err) => {
                        toasts.push(api_error_to_toast(&err, "deleting recipe image"));
                    }
                },
            };
        }
    });

    let on_image_change = move |ev: Event| {
        let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
        let file = input.files().unwrap().get(0).unwrap();
        image_file.set(Some(file));
    };

    view! {cx,
        <ModalSaveCancel
            title="Edit Image".to_owned()
            loading=save_action.pending()
            on_save=move || save_action.dispatch(())
            on_cancel=move || on_action(None)
        >
            <div class="form-control mb-2">
                <span class="label">"Upload New Image"</span>
                <label class="join">
                    <span class="label p-3 bg-base-300 join-item">"Upload"</span>
                    <input
                        on:change=on_image_change
                        type="file"
                        class="file-input file-input-bordered w-full join-item"
                        accept="image/*"
                        multiple=false
                    />
                </label>
            </div>
            {move || {
                if image_id.get().is_some() {
                    return Some(view!{cx,
                        <div class="form-control mb-2">
                            <span class="label">"Remove Existing"</span>
                            <button
                                on:click=move |_| save_action.dispatch(())
                                class="btn btn-outline btn-error"
                                type="button"
                            >
                                "Delete Existing (Permanent)"
                            </button>
                        </div>
                    })
                }
                None
            }}
        </ModalSaveCancel>
    }
}
