use crate::{
    components::input::BaseUrlInput,
    contexts::{
        login::CurrentLogin,
        prelude::{use_login, use_toasts},
    },
    helpers::api_error_to_toast,
};
use leptos::{ev::SubmitEvent, leptos_dom::helpers::location, *};
use leptos_router::A;
use mcc_frontend_core::{api::Api, APP_TITLE};
use mcc_frontend_types::{Login, StoredLogin};

#[component]
pub fn Login() -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login();
    let toasts = use_toasts();

    let base_url = create_rw_signal::<Option<String>>(location().origin().ok());
    let (username, set_username) = create_signal(String::default());
    let (password, set_password) = create_signal(String::default());

    let is_loading = create_rw_signal(false);

    let fetch_token = move |base_url: String, details: Login| {
        async move {
            is_loading.set(true);
            let api_url = format!("{}/api", base_url);
            let media_url = format!("{}/media", base_url);
            let api = Api::new(api_url.clone(), None);
            // request oauth token, with given details
            match api.post_login(&details).await {
                Ok(token) => {
                    log::debug!("login successful, token will expire at: {:?}", token.expiry);
                    batch(move || {
                        set_login.set(Some(StoredLogin {
                            api_url,
                            media_url,
                            token,
                        }));
                        is_loading.set(false);
                    });
                }
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "authenticating login"));
                    is_loading.set(false);
                }
            };
        }
    };

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        if let Some(base_url) = base_url.get() {
            spawn_local(fetch_token(
                base_url,
                Login {
                    username: username.get(),
                    password: password.get(),
                },
            ));
        }
    };

    view! {
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="card flex-shrink-0 w-full max-w-md shadow-2xl bg-base-100">
                    <div class="card-body">
                        <img class="mb-2 mx-auto w-36 drop-shadow-lg" src="/public/icon.svg" alt="'My Cooking Codex' Icon" />
                        <div>
                            <h1 class="text-5xl font-bold mb-4">{APP_TITLE}</h1>
                            <p class="py-6">"Login here."</p>
                        </div>
                        <form on:submit=on_submit>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">"API Server"</span></label>
                                <BaseUrlInput
                                    value=base_url.get()
                                    on_change=move |v| base_url.set(v)
                                />
                            </div>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">"Username"</span></label>
                                <input
                                    prop:value=move || username.get()
                                    on:input=move |ev| {set_username.set(event_target_value(&ev))}
                                    type="text"
                                    class="input input-bordered"
                                    placeholder="e.g. leo"
                                    autocomplete="username"
                                    spellcheck=false
                                    required=true
                                />
                            </div>
                            <div class="form-control mb-6">
                                <label class="label"><span class="label-text">"Password"</span></label>
                                <input
                                    prop:value=move || password.get()
                                    on:input=move |ev| {set_password.set(event_target_value(&ev))}
                                    type="password"
                                    class="input input-bordered"
                                    placeholder="e.g. ••••••••"
                                    autocomplete="current-password"
                                    required=true
                                />
                            </div>
                            <div class="form-control join join-vertical w-full">
                                <button
                                    class="btn btn-primary join-item"
                                    // class="loading"
                                    class:loading=move || is_loading.get()
                                    type="submit"
                                    prop:disabled=move || base_url.get().is_none()
                                >
                                    "Login"
                                </button>
                                <A href="/signup" class="btn join-item">{"Signup Instead?"}</A>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
