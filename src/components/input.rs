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
    let (edit_mode, set_edit_mode) = create_signal(cx, bool::default());

    let on_mode_click = move |_| {
        let edit_mode = edit_mode.get();
        if edit_mode {
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
        } else {
            // in edit mode, so it's unsaved
            new_base_url.set(None);
        }
        set_edit_mode.set(!edit_mode);
    };

    view! {cx,
        <div class="input-group">
            {move || {
                view!(cx,
                    <input
                        prop:value={move || {
                            if edit_mode.get() {
                                base_url.get().unwrap_or_else(|| "".to_owned())
                            } else {
                                preview_base_url.get()
                            }

                        }}
                        on:input={move |ev| { set_base_url.set(Some(event_target_value(&ev))) }}
                        type="url"
                        class="input w-full"
                        class:input-bordered=move || edit_mode.get()
                        class:input-sm=move || !edit_mode.get()
                        placeholder="https://"
                        required=true
                        readonly=move || !edit_mode.get()
                    />
                    <button
                        on:click=on_mode_click
                        type="button"
                        class="btn"
                        class:btn-sm=move || !edit_mode.get()
                    >
                        {move || {
                            if edit_mode.get() {
                                "Save"
                            } else {
                                "Change"
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
