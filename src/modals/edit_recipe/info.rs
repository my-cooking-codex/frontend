use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::{ModalSaveCancel, ModalSaveCancelProps},
};
use leptos::*;
use mcc_frontend_types::recipe::{Info, InfoYields, UpdateInfo, UpdateRecipe};

#[component]
pub fn EditInfoModal<F>(cx: Scope, id: String, info: Info, on_action: F) -> impl IntoView
where
    F: Fn(Option<Info>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let info = create_rw_signal(cx, info);

    let update_recipe = create_action(cx, move |_: &()| {
        let id = id.clone();
        let info = info.get();
        let api = api.get().expect("api expected to be set");
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        info: UpdateInfo {
                            yields: info.yields.clone(),
                        },
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(info)),
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "saving recipe info"));
                }
            }
        }
    });

    view! {cx,
        <ModalSaveCancel
            title="Edit Info".to_owned()
            loading=update_recipe.pending()
            on_save=move || update_recipe.dispatch(())
            on_cancel=move || on_action(None)
        >
            <h2 class="text-lg mb-2">"Serving Size"</h2>
            <div class="flex">
                <label class="input-group w-auto">
                    <span>"Amount"</span>
                    <input
                        prop:value=move || info.get().yields.unwrap_or_else(|| InfoYields { value: 0, unit_type: "".to_owned() }).value
                        on:input=move |ev| {
                            info.update(|info| {
                                info.yields = Some(InfoYields {
                                    value: event_target_value(&ev).parse().unwrap_or(0),
                                    unit_type: info.yields.clone().unwrap_or_else(|| InfoYields { value: 0, unit_type: "".to_owned() }).unit_type,
                                });
                            })
                        }
                        type="number"
                        class="input input-bordered w-24"
                        min=1 required=true
                    />
                </label>
                <label class="input-group">
                    <span>"Type"</span>
                    <input
                        prop:value=move || info.get().yields.unwrap_or_else(|| InfoYields { value: 0, unit_type: "".to_owned() }).unit_type
                        on:input=move |ev| {
                            info.update(|info| {
                                info.yields = Some(InfoYields {
                                    value: info.yields.clone().unwrap_or_else(|| InfoYields { value: 0, unit_type: "".to_owned() }).value,
                                    unit_type: event_target_value(&ev),
                                });
                            })
                        }
                        type="text"
                        class="input input-bordered w-full"
                        list="units" required=true
                    />
                    <datalist id="units">
                        <option value="servings" />
                        <option value="g" />
                        <option value="kg" />
                        <option value="ml" />
                        <option value="l" />
                        <option value="tsp" />
                        <option value="tbsp" />
                        <option value="cup" />
                        <option value="oz" />
                        <option value="lb" />
                        <option value="can" />
                        <option value="bottle" />
                        <option value="jar" />
                    </datalist>
                </label>
            </div>
        </ModalSaveCancel>
    }
}
