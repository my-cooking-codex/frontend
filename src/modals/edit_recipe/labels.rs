use leptos::*;
use mcc_frontend_types::recipe::UpdateRecipe;
use std::collections::HashSet;

use crate::{
    components::input::LabelSelector,
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalSaveCancel,
};

#[component]
pub fn EditLabelsModal<F>(cx: Scope, id: String, labels: Vec<String>, on_action: F) -> impl IntoView
where
    F: Fn(Option<Vec<String>>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);

    let existing_labels = create_resource(
        cx,
        || {},
        move |_| async move {
            let api = api.get_untracked().expect("api expected to be set");
            match api.get_labels().await {
                Ok(labels) => HashSet::from_iter(labels.into_iter()),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "loading labels"));
                    HashSet::new()
                }
            }
        },
    );
    let labels: leptos::RwSignal<HashSet<std::string::String>> =
        create_rw_signal(cx, HashSet::from_iter(labels.into_iter()));

    let update_labels = create_action(cx, move |_: &()| {
        let id = id.clone();
        let api = api.get_untracked().expect("api expected to be set");
        let labels = labels.get_untracked().into_iter().collect::<Vec<String>>();
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        labels: Some(labels.clone()),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(labels)),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "saving recipe labels"));
                }
            }
        }
    });

    let on_change = move |selected| {
        labels.set(selected);
    };

    view! {cx,
        <ModalSaveCancel
            title="Edit Labels"
            loading=update_labels.pending()
            on_save=move || update_labels.dispatch(())
            on_cancel=move || on_action(None)
        >
            <LabelSelector
                labels=Signal::derive(cx, move || existing_labels.read(cx).unwrap_or_default())
                allow_new=true
                selected=labels
                on_change=on_change
            />
        </ModalSaveCancel>
    }
}
