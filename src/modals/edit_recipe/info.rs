use crate::{
    components::input::HmsInput,
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::ModalSaveCancel,
};
use leptos::*;
use mcc_frontend_types::{
    recipe::{Info, InfoYields, UpdateRecipe},
    HourMinuteSecond,
};

#[component]
pub fn EditInfoModal<F>(id: String, info: Info, on_action: F) -> impl IntoView
where
    F: Fn(Option<Info>) + 'static + Copy,
{
    let toasts = use_toasts();
    let CurrentApi { api, .. } = use_api();
    let info = create_rw_signal(info);

    let update_recipe = create_action(move |_: &()| {
        let id = id.clone();
        let info = info.get_untracked();
        let api = api.get_untracked().expect("api expected to be set");
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

    view! {
        <ModalSaveCancel
            title="Edit Info"
            loading=update_recipe.pending()
            on_save=move || update_recipe.dispatch(())
            on_cancel=move || on_action(None)
        >
            // yields
            <div class="my-4">
                <h2 class="text-lg mb-2">"Serving Size"</h2>
                <div class="flex gap-1">
                    <label class="join w-auto">
                        <span class="label p-3 bg-base-300 join-item">"Amount"</span>
                        <input
                            prop:value=move || info.get().yields.unwrap_or_default().value
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse() {
                                    info.update(|info| {
                                        info.yields = Some(InfoYields {
                                            value,
                                            unit_type: info.yields.clone().unwrap_or_default().unit_type,
                                        });
                                    })
                                }
                            }
                            type="number"
                            class="input input-bordered w-24 join-item"
                            min=1 required=true
                        />
                    </label>
                    <label class="join">
                        <span class="label p-3 bg-base-300 join-item">"Type"</span>
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
                            class="input input-bordered join-item w-full"
                            list="units"
                            placeholder="e.g. servings"
                            required=true
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
                        value=Signal::derive( move || HourMinuteSecond::from_secs(info.get().prep_time))
                        on_input=move |value| info.update(|info| info.prep_time = value.as_secs())
                        required=true
                    />
                    <span class="my-2">"Cook Time"</span>
                    <HmsInput
                        value=Signal::derive( move || HourMinuteSecond::from_secs(info.get().cook_time))
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
            // source
            <div class="my-4">
                <h2 class="text-lg mb-2">"Recipe Source"</h2>
                <label class="join w-full">
                    <span class="label p-3 bg-base-300 join-item">"Source"</span>
                    <input
                        prop:value=move || info.get().source.unwrap_or_default()
                        on:input=move |ev| {
                            info.update(|info| {
                                info.source = Some(event_target_value(&ev));
                            })
                        }
                        type="text"
                        class="input input-bordered join-item w-full"
                        placeholder="e.g. Mom's Recipe Book"
                    />
                </label>
            </div>
        </ModalSaveCancel>
    }
}
