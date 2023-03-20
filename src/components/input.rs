use leptos::*;
use mcc_frontend_core::api::sanitise_base_url;
use url::Url;

fn make_preview_url(base_url: &str) -> Option<String> {
    let url = Url::parse(base_url).ok()?;
    Some(url.host_str().unwrap().to_owned())
}

#[component]
pub fn BaseUrlInput(
    cx: Scope,
    #[prop(into)] value: Option<String>,
    #[prop(into)] new_base_url: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (base_url, set_base_url) = create_signal(cx, value);
    let (preview_base_url, set_preview_base_url) = create_signal(
        cx,
        make_preview_url(base_url.get().as_deref().unwrap_or_default())
            .unwrap_or_else(|| "(unset)".to_owned()),
    );
    let (edit_mode, set_edit_mode) = create_signal(cx, bool::default());

    let on_mode_click = move |_| {
        let current_mode = edit_mode.get();
        if current_mode {
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
        }
        set_edit_mode.set(!current_mode);
    };

    view! {cx,
        <div class="form-control">
            <div class="input-group">
                {move || {
                    if edit_mode.get() {
                        view!(cx,
                            <input
                                prop:value={move || base_url.get()}
                                on:input={move |ev| { set_base_url.set(Some(event_target_value(&ev))) }}
                                type="url"
                                class="input input-bordered"
                                placeholder="https://"
                                required=true
                            />
                            <button on:click=on_mode_click type="button" class="btn">"Save"</button>
                        )
                    } else {
                        view!(cx,
                            <span>"Using Server At: "</span>
                            <span>{move || preview_base_url.get()}</span>
                            <button on:click=on_mode_click type="button" class="btn">"Change"</button>
                        )
                    }
                }}
            </div>
        </div>
    }
}
