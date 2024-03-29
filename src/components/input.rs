use leptos::*;
use mcc_frontend_core::api::sanitise_base_url;
use mcc_frontend_types::{Fraction, HourMinuteSecond};
use regex::Regex;
use std::collections::HashSet;
use url::Url;

// matches whole numbers or fractions with or without a mixed number
const VALID_FRACTIONAL_INPUT_REGEX: &str =
    r#"^(?:(?:\d+)|(?:(?:[1-9]\d* )?\d+/\d+)|(?:\d+\.\d+))$"#;
// fractions with or without a mixed number
const VALID_FRACTION_REGEX: &str = r#"^(?:(?:[1-9]\d* )?\d+/\d+)$"#;

fn base_url_valid(base_url: &str) -> bool {
    Url::parse(base_url).is_ok()
}

fn make_preview_url(base_url: &str) -> Option<String> {
    let url = Url::parse(base_url).ok()?;
    Some(url.host_str().unwrap().to_owned())
}

#[component]
pub fn BaseUrlInput<F>(value: Option<String>, on_change: F) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy,
{
    let (base_url, set_base_url) = create_signal(value);
    let preview_base_url = Signal::derive(move || {
        let base_url = base_url.get().unwrap_or_default();
        make_preview_url(&base_url).unwrap_or_else(|| "(unset)".to_owned())
    });

    let is_edit_mode = create_rw_signal(bool::default());

    let view_mode = move || {
        if let Some(url) = base_url.get() {
            let sanitised = sanitise_base_url(url);
            match base_url_valid(&sanitised) {
                true => {
                    on_change(Some(sanitised.clone()));
                    set_base_url.set(Some(sanitised.clone()));
                }
                false => {
                    on_change(None);
                    set_base_url.set(None);
                }
            };
        }
    };

    let edit_mode = move || {
        on_change(None);
    };

    let on_switch_mode_click = move |_| {
        // reverse state
        is_edit_mode.update(|is_edit_mode| *is_edit_mode = !*is_edit_mode);
        match is_edit_mode.get() {
            true => edit_mode(),
            false => view_mode(),
        };
    };

    view! {
        <div class="join">
            {move || {
                view!(
                    <input
                        prop:value={move || {
                            if is_edit_mode.get() {
                                base_url.get().unwrap_or_else(|| "".to_owned())
                            } else {
                                preview_base_url.get()
                            }

                        }}
                        on:input=move |ev| { set_base_url.set(Some(event_target_value(&ev))) }
                        type="url"
                        class="input w-full join-item"
                        class:input-bordered=move || is_edit_mode.get()
                        class:input-sm=move || !is_edit_mode.get()
                        placeholder="https://"
                        required=true
                        readonly=move || !is_edit_mode.get()
                    />
                    <button
                        on:click=on_switch_mode_click
                        type="button"
                        class="btn join-item"
                        class:btn-sm=move || !is_edit_mode.get()
                        class:btn-primary=move || is_edit_mode.get()
                    >
                        {move || {
                            if is_edit_mode.get() {
                                "Save"
                            } else {
                                "Edit"
                            }
                        }}
                    </button>
                )
            }}
        </div>
    }
}

#[component]
pub fn FractionalInput<F>(
    class: String,
    value: f32,
    on_input: F,
    required: bool,
    placeholder: String,
) -> impl IntoView
where
    F: Fn(f32) + 'static + Copy,
{
    let invalid = create_rw_signal(false);
    let input_value = create_rw_signal(value.to_string());

    let on_value_input = move |ev| {
        let input = event_target_value(&ev);
        if Regex::new(VALID_FRACTIONAL_INPUT_REGEX)
            .unwrap()
            .is_match(&input)
        {
            let parsed: f32;
            // if the input is a valid fractional number, update the state
            if Regex::new(VALID_FRACTION_REGEX).unwrap().is_match(&input) {
                // parse input is a valid fraction, convert it to a float
                let raw_float: f32 = input
                    .parse::<Fraction>()
                    .expect("Failed to parse fraction")
                    .into();
                // 'round' float to 4 decimal places. (4 is needed for 1/3)
                const DECIMAL_PLACES: f32 = 1000.0;
                parsed = (raw_float * DECIMAL_PLACES).round() / DECIMAL_PLACES;
                on_input(parsed);
                // on_input(raw_float);
            } else {
                // parse input as a float, with or without decimal place
                parsed = input.parse::<f32>().expect("Failed to parse float");
                on_input(parsed);
            }
            invalid.set(false);
        } else {
            invalid.set(true);
        }
    };

    view! {
        <input
            prop:value=move || input_value.get()
            on:input=on_value_input
            type="text"
            class=class
            // class="input-error" // ! needed for tailwind to include the css !
            class:input-error=move || invalid.get()
            pattern={VALID_FRACTIONAL_INPUT_REGEX}
            required=required
            placeholder=placeholder
        />
    }
}

#[component]
pub fn HmsInput<F>(
    #[prop(into)] value: Signal<HourMinuteSecond>,
    on_input: F,
    required: bool,
) -> impl IntoView
where
    F: Fn(HourMinuteSecond) + 'static + Copy,
{
    view! {
        <div class="flex gap-1">
            <label class="join">
                <input
                    prop:value=move || value.get().hours.to_string()
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        if !input.is_empty() {
                            let parsed = input.parse::<usize>().expect("Failed to parse usize");
                            let mut value = value.get();
                            value.hours = parsed;
                            on_input(value);
                        }
                    }
                    type="number"
                    class="input input-bordered w-full text-right join-item"
                    min=0 required=required
                />
                <span class="label p-3 bg-base-300 join-item">"H"</span>
            </label>
            <label class="join">
                <input
                    prop:value=move || value.get().minutes.to_string()
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        if !input.is_empty() {
                            let parsed = input.parse::<usize>().expect("Failed to parse usize");
                            let mut value = value.get();
                            value.minutes = parsed;
                            on_input(value);
                        }
                    }
                    type="number"
                    class="input input-bordered w-full text-right join-item"
                    min=0 required=required
                />
                <span class="label p-3 bg-base-300 join-item">"M"</span>
            </label>
            <label class="join">
                <input
                    prop:value=move || value.get().seconds.to_string()
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        if !input.is_empty() {
                            let parsed = input.parse::<usize>().expect("Failed to parse usize");
                            let mut value = value.get();
                            value.seconds = parsed;
                            on_input(value);
                        }
                    }
                    type="number"
                    class="input input-bordered w-full text-right join-item"
                    min=0 required=required
                />
                <span class="label p-3 bg-base-300 join-item">"S"</span>
            </label>
        </div>
    }
}

#[component]
pub fn DropdownConfirm<F>(
    #[prop(into)] title: String,
    #[prop(into)] confirm_aria: String,
    on_confirm: F,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    view! {
        <div class="dropdown dropdown-bottom dropdown-end".to_owned() + &class.map_or("".to_owned(),|v| format!(" {v}"))>
            <label tabindex="0" class="btn">{title}</label>
            <div class="dropdown-content menu bg-base-200 rounded">
                <button
                    on:click=move |_| on_confirm()
                    class="btn btn-outline btn-error"
                    tabindex="0"
                    aria-label={confirm_aria}>
                    "Confirm"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn ThreeStateSelect<F>(
    #[prop(into)] value: Signal<Option<bool>>,
    on_change: F,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView
where
    F: Fn(Option<bool>) + 'static + Copy,
{
    view! {
        <select
            on:change=move |ev| {
                match event_target_value(&ev).as_str() {
                    "1" => on_change(Some(true)),
                    "0" => on_change(Some(false)),
                    _ => on_change(None),
                }
            }
            class=class
        >
            <option
                prop:selected=move || value.get().is_none()
                value=""
            >"Any"</option>
            <option
                prop:selected=move || value.get().unwrap_or(false)
                value="1"
            >"Yes"</option>
            <option
                prop:selected=move || !value.get().unwrap_or(true)
                value="0"
            >"No"</option>
        </select>
    }
}

#[component]
pub fn LabelSelector<F>(
    #[prop(into)] labels: Signal<HashSet<String>>,
    allow_new: bool,
    #[prop(optional)] compact: bool,
    #[prop(into)] selected: Signal<HashSet<String>>,
    on_change: F,
) -> impl IntoView
where
    F: Fn(HashSet<String>) + 'static + Copy,
{
    let label_input = create_rw_signal(String::default());
    let label_input_invalid = Signal::derive(move || {
        if !label_input.get().is_empty() && !allow_new && !labels.get().contains(&label_input.get())
        {
            return true;
        }
        false
    });

    let on_add = move || {
        if !label_input_invalid.get() && !label_input.get().is_empty() {
            let mut new_selected = selected.get();
            new_selected.insert(label_input.get());
            on_change(new_selected);
            label_input.set(String::default());
        }
    };

    let on_delete = move |name: &str| {
        let mut new_selected = selected.get();
        new_selected.remove(name);
        on_change(new_selected);
    };

    view! {
        <div class="flex flex-col gap-2">
            <div class="flex flex-wrap gap-2 overflow-y-auto">
                <For
                    each=move || selected.get().into_iter()
                    key=|name| name.to_owned()
                    children=move |label_name| {
                        view!{
                        <div class="inline-flex items-center bg-primary p-1 gap-2 rounded-lg shadow-md">
                            <span class="text-primary-content">{&label_name}</span>
                            <button
                                aria-label=format!("remove label '{}'", &label_name)
                                on:click=move |_| on_delete(&label_name)
                                class="btn btn-sm"
                            >
                                "X"
                            </button>
                        </div>
                        }
                    }
                />
            </div>
                <div class="join w-full">
                    <input
                        prop:value=move || label_input.get()
                        on:input=move |ev| {label_input.set(event_target_value(&ev))}
                        on:keydown=move|ev| {
                            if ev.key_code() == 13 {
                                ev.prevent_default();
                                on_add();
                            }
                        }
                        class="input input-bordered w-full join-item"
                        // class="input-sm"
                        class:input-sm=compact
                        // class="input-error"
                        class:input-error=move || label_input_invalid.get()
                        type="text"
                        placeholder="e.g. High Protein..."
                        list="labels"
                        maxlength="60"
                    />
                    <datalist id="labels">
                        {move || {
                            let selected = selected.get();
                            labels.get().difference(&selected).map(|label|
                                view! {<option value=label />}
                            ).collect_view()
                        }}
                    </datalist>
                    <button
                        on:click=move |_| on_add()
                        class="btn join-item"
                        // class="btn-sm"
                        class:btn-sm=compact
                        type="button"
                    >
                        "Add"
                    </button>
                </div>
        </div>
    }
}
