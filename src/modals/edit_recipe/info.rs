use crate::{
    components::input::{HmsInput, HmsInputProps},
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::{ModalSaveCancel, ModalSaveCancelProps},
};
use leptos::*;
use mcc_frontend_types::{
    recipe::{Info, InfoYields, UpdateRecipe},
    HourMinuteSecond,
};

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
                        info: info.clone(),
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
            // yields
            <div class="my-4">
                <h2 class="text-lg mb-2">"Serving Size"</h2>
                <div class="flex">
                    <label class="input-group w-auto">
                        <span>"Amount"</span>
                        <input
                            prop:value=move || info.get().yields.unwrap_or_default().value
                            on:input=move |ev| {
                                info.update(|info| {
                                    info.yields = Some(InfoYields {
                                        value: event_target_value(&ev).parse().unwrap_or(0),
                                        unit_type: info.yields.clone().unwrap_or_default().unit_type,
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
                            prop:value=move || info.get().yields.unwrap_or_default().unit_type
                            on:input=move |ev| {
                                info.update(|info| {
                                    info.yields = Some(InfoYields {
                                        value: info.yields.clone().unwrap_or_default().value,
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
            </div>
            // times
            <div class="my-4">
                <h2 class="text-lg mb-1">"Times"</h2>
                <div class="form-control">
                    <span class="mb-2">"Prep Time"</span>
                    <HmsInput
                        value=move || HourMinuteSecond::from_secs(info.get().prep_time)
                        on_input=move |value| info.update(|info| info.prep_time = value.as_secs())
                        required=true
                    />
                    <span class="my-2">"Cook Time"</span>
                    <HmsInput
                        value=move || HourMinuteSecond::from_secs(info.get().cook_time)
                        on_input=move |value| info.update(|info| info.cook_time = value.as_secs())
                        required=true
                    />
                </div>
            </div>
            // flags
            <div class="my-4">
                <h2 class="text-lg mb-2">"Flags"</h2>
                <div class="form-control">
                    <label class="label cursor-pointer">
                        <span>"Freezable"</span>
                        <input
                            prop:checked=move || info.get().freezable
                            on:input=move |ev| {
                                info.update(|info| {
                                    info.freezable = event_target_checked(&ev);
                                })
                            }
                            type="checkbox"
                            class="checkbox"
                        />
                    </label>
                </div>
                <div class="form-control">
                    <label class="label cursor-pointer">
                        <span>"Microwave Only"</span>
                        <input
                            prop:checked=move || info.get().microwave_only
                            on:input=move |ev| {
                                info.update(|info| {
                                    info.microwave_only = event_target_checked(&ev);
                                })
                            }
                            type="checkbox"
                            class="checkbox"
                        />
                    </label>
                </div>
            </div>
        </ModalSaveCancel>
    }
}
