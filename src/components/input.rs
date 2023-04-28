use leptos::*;
use mcc_frontend_core::api::sanitise_base_url;
use mcc_frontend_types::{Fraction, HourMinuteSecond};
use regex::Regex;
use url::Url;

// matches whole numbers or fractions with or without a mixed number
const VALID_FRACTIONAL_INPUT_REGEX: &str =
    r#"^(?:(?:\d+)|(?:(?:[1-9]\d* )?\d+/\d+)|(?:\d+\.\d+))$"#;
// fractions with or without a mixed number
const VALID_FRACTION_REGEX: &str = r#"^(?:(?:[1-9]\d* )?\d+/\d+)$"#;

fn make_preview_url(base_url: &str) -> Option<String> {
    let url = Url::parse(base_url).ok()?;
    Some(url.host_str().unwrap().to_owned())
}

#[component]
pub fn BaseUrlInput(
    cx: Scope,
    value: Option<String>,
    new_base_url: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (base_url, set_base_url) = create_signal(cx, value);
    let (preview_base_url, set_preview_base_url) = create_signal(
        cx,
        make_preview_url(base_url.get().as_deref().unwrap_or_default())
            .unwrap_or_else(|| "(unset)".to_owned()),
    );
    let is_edit_mode = create_rw_signal(cx, bool::default());

    let view_mode = move || {
        if let Some(url) = base_url.get() {
            let sanitised = sanitise_base_url(url);
            let preview = match make_preview_url(&sanitised) {
                Some(v) => {
                    new_base_url.set(Some(sanitised.clone()));
                    set_base_url.set(Some(sanitised.clone()));
                    v
                }
                None => {
                    new_base_url.set(None);
                    set_base_url.set(None);
                    "(unset)".to_owned()
                }
            };
            set_preview_base_url.set(preview);
        }
    };

    let edit_mode = move || {
        new_base_url.set(None);
    };

    let on_switch_mode_click = move |_| {
        // reverse state
        is_edit_mode.update(|is_edit_mode| *is_edit_mode = !*is_edit_mode);
        match is_edit_mode.get() {
            true => edit_mode(),
            false => view_mode(),
        };
    };

    view! {cx,
        <div class="input-group">
            {move || {
                view!(cx,
                    <input
                        prop:value={move || {
                            if is_edit_mode.get() {
                                base_url.get().unwrap_or_else(|| "".to_owned())
                            } else {
                                preview_base_url.get()
                            }

                        }}
                        on:input={move |ev| { set_base_url.set(Some(event_target_value(&ev))) }}
                        type="url"
                        class="input w-full"
                        class:input-bordered=move || is_edit_mode.get()
                        class:input-sm=move || !is_edit_mode.get()
                        placeholder="https://"
                        required=true
                        readonly=move || !is_edit_mode.get()
                    />
                    <button
                        on:click=on_switch_mode_click
                        type="button"
                        class="btn"
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
    cx: Scope,
    class: String,
    value: f32,
    on_input: F,
    required: bool,
    placeholder: String,
) -> impl IntoView
where
    F: Fn(f32) + 'static + Copy,
{
    let invalid = create_rw_signal(cx, false);
    let input_value = create_rw_signal(cx, value.to_string());

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

    view! {cx,
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
pub fn HmsInput<S, F>(cx: Scope, value: S, on_input: F, required: bool) -> impl IntoView
where
    S: Fn() -> HourMinuteSecond + 'static + Copy,
    F: Fn(HourMinuteSecond) + 'static + Copy,
{
    let hms = create_rw_signal(cx, value().simplify());

    create_effect(cx, move |_| {
        let hms = hms.get();
        on_input(hms);
    });

    view! {cx,
        <div class="flex">
            <label class="input-group">
                <input
                    prop:value=move || hms.get().hours.to_string()
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        if !input.is_empty() {
                            let parsed = input.parse::<usize>().expect("Failed to parse usize");
                            hms.update(|hms| hms.hours = parsed);
                        }
                    }
                    type="number"
                    class="input input-bordered w-full text-right"
                    min=0 required=required
                />
                <span>"H"</span>
            </label>
            <label class="input-group">
                <input
                    prop:value=move || hms.get().minutes.to_string()
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        if !input.is_empty() {
                            let parsed = input.parse::<usize>().expect("Failed to parse usize");
                            hms.update(|hms| hms.minutes = parsed);
                        }
                    }
                    type="number"
                    class="input input-bordered w-full text-right"
                    min=0 required=required
                />
                <span>"M"</span>
            </label>
            <label class="input-group">
                <input
                    prop:value=move || hms.get().seconds.to_string()
                    on:input=move |ev| {
                        let input = event_target_value(&ev);
                        if !input.is_empty() {
                            let parsed = input.parse::<usize>().expect("Failed to parse usize");
                            hms.update(|hms| hms.seconds = parsed);
                        }
                    }
                    type="number"
                    class="input input-bordered w-full text-right"
                    min=0 required=required
                />
                <span>"S"</span>
            </label>
        </div>
    }
}

#[component]
pub fn DropdownConfirm<F>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] confirm_aria: String,
    on_confirm: F,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView
where
    F: Fn() -> () + 'static + Copy,
{
    view! {cx,
        <div class="dropdown dropdown-bottom".to_owned() + &class.map_or("".to_owned(),|v| format!(" {v}"))>
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
    cx: Scope,
    value: Option<bool>,
    on_input: F,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView
where
    F: Fn(Option<bool>) + 'static + Copy,
{
    let value = create_rw_signal(cx, value);

    create_effect(cx, move |_| {
        let value = value.get();
        on_input(value);
    });

    view! {cx,
        <select
            on:change=move |ev| {
                match event_target_value(&ev).as_str() {
                    "1" => value.set(Some(true)),
                    "0" => value.set(Some(false)),
                    _ => value.set(None),
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
