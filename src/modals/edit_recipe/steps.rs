use crate::{
    contexts::prelude::{use_api, use_toasts, CurrentApi},
    helpers::api_error_to_toast,
    modals::base::{ModalSaveCancel, ModalSaveCancelProps},
};
use leptos::*;
use mcc_frontend_types::recipe::{Step, UpdateRecipe};

enum EditStepEvent {
    Update(Step),
    MoveUp,
    MoveDown,
    Delete,
}

#[component]
fn EditStep<F>(cx: Scope, index: usize, len: usize, step: Step, on_event: F) -> impl IntoView
where
    F: Fn(usize, EditStepEvent) + 'static + Copy,
{
    let step = create_rw_signal(cx, step);

    create_effect(cx, move |_| {
        on_event(index, EditStepEvent::Update(step.get()));
    });

    view! {cx,
        <li class="mb-4 p-4 rounded bg-base-200">
            <div class="flex mb-2">
                <input
                    on:input=move |ev| step.update(|s| s.title = {
                        let value = event_target_value(&ev);
                        if value.is_empty() {
                            None
                        } else {
                            Some(value)
                        }
                    })
                    prop:value=move || step.get().title
                    type="text"
                    class="input input-bordered w-full mr-2"
                    placeholder={format!("Step {}", index+1)}
                />
                <div class="btn-group">
                    {
                        if index == 0 {
                            view!{cx, <button type="button" class="btn btn-disabled">"Up"</button>}
                        } else {
                            view!{cx,
                                <button
                                    on:click=move |_| on_event(index, EditStepEvent::MoveUp)
                                    type="button"
                                    class="btn">
                                    "Up"
                                </button>
                            }
                        }
                    }
                    {
                        if len == index+1 {
                            view!{cx, <button type="button" class="btn btn-disabled">"Down"</button>}
                        } else {
                            view!{cx,
                                <button
                                    on:click=move |_| on_event(index, EditStepEvent::MoveDown)
                                    type="button"
                                    class="btn">
                                    "Down"
                                </button>
                            }
                        }
                    }
                    <button
                        on:click=move |_| on_event(index, EditStepEvent::Delete)
                        type="button"
                        class="btn">
                        "X"
                    </button>
                </div>
            </div>
            <textarea
                on:input=move |ev| step.update(|s| s.description = event_target_value(&ev))
                prop:value=move || step.get().description
                class="textarea textarea-bordered w-full"
                placeholder="Description here..."
                required=true
            />
        </li>
    }
}

#[component]
pub fn EditStepsModal<F>(cx: Scope, id: String, steps: Vec<Step>, on_action: F) -> impl IntoView
where
    F: Fn(Option<Vec<Step>>) + 'static + Copy,
{
    let toasts = use_toasts(cx);
    let CurrentApi { api, .. } = use_api(cx);
    let steps = create_rw_signal(cx, steps);

    let update_recipe = create_action(cx, move |_: &()| {
        let id = id.clone();
        let api = api.get().expect("api expected to be set");
        let steps = steps.get();
        async move {
            match api
                .patch_update_recipe(
                    id,
                    &UpdateRecipe {
                        steps: Some(steps.iter().map(|s| s.clone().into()).collect()),
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(_) => on_action(Some(steps.clone())),
                Err(err) => toasts.push(api_error_to_toast(&err, "updating steps")),
            }
        }
    });

    let on_step_event = move |index: usize, ev: EditStepEvent| match ev {
        EditStepEvent::Update(step) => {
            // XXX this is a bit hacky. Although it works until each step has a unique id
            steps.update_untracked(|steps| steps[index] = step);
        }
        EditStepEvent::MoveUp => {
            steps.update(|steps| {
                steps.swap(index, index - 1);
            });
        }
        EditStepEvent::MoveDown => {
            steps.update(|steps| {
                steps.swap(index, index + 1);
            });
        }
        EditStepEvent::Delete => {
            steps.update(|steps| {
                steps.remove(index);
            });
        }
    };

    view! {cx,
        <ModalSaveCancel
            title="Edit Steps".to_owned()
            loading=update_recipe.pending()
            on_save=move || update_recipe.dispatch(())
            on_cancel=move || on_action(None)
        >
            <div class="max-h-[50vh] lg:max-h-[60vh] overflow-y-auto">
                <ol>
                    // TODO each step should have it's own unique id,
                    // preventing all steps from being updated on a single change
                    {move || {
                        let steps = steps.get();
                        steps.iter().enumerate().map(|(i, step)| {
                            view!{cx, <EditStep
                                len=steps.len()
                                index=i
                                step=step.clone()
                                on_event=on_step_event
                            />}
                        }).collect::<Vec<_>>()
                    }}
                </ol>
                <button
                    on:click=move |_| steps.update(|steps| steps.push(Step::default()))
                    type="button"
                    class="btn w-full">
                    "Add Step"
                </button>
            </div>
        </ModalSaveCancel>
    }
}
