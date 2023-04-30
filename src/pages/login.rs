use crate::{
    components::input::{BaseUrlInput, BaseUrlInputProps},
    contexts::{
        login::CurrentLogin,
        prelude::{use_api, use_login, use_toasts, CurrentApi},
    },
    helpers::api_error_to_toast,
};
use leptos::{ev::SubmitEvent, leptos_dom::helpers::location, *};
use leptos_router::{AProps, A};
use mcc_frontend_core::{api::Api, APP_TITLE};
use mcc_frontend_types::{Login, StoredLogin};

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login(cx);
    let CurrentApi { set_api, .. } = use_api(cx);
    let toasts = use_toasts(cx);

    let (base_url, set_base_url) = create_signal::<Option<String>>(cx, location().origin().ok());
    let (username, set_username) = create_signal(cx, String::default());
    let (password, set_password) = create_signal(cx, String::default());

    let fetch_token = create_action(cx, move |args: &(String, Login)| {
        let (base_url, details) = args.to_owned();
        async move {
            let api_url = format!("{}/api", base_url);
            let media_url = format!("{}/media", base_url);
            let api = Api::new(api_url.clone(), None);
            // request oauth token, with given details
            match api.post_login(&details).await {
                Ok(token) => {
                    log::debug!("login successful, token will expire at: {:?}", token.expiry);
                    set_api.set(Some(Api::new(api_url.clone(), Some(token.clone()))));
                    set_login.set(Some(StoredLogin {
                        api_url,
                        media_url,
                        token,
                    }));
                }
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "authenticating login"));
                }
            };
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        if let Some(base_url) = base_url.get() {
            fetch_token.dispatch((
                base_url,
                Login {
                    username: username.get(),
                    password: password.get(),
                },
            ));
        }
    };

    view! {cx,
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                    <div class="card-body">
                        <div class="mb-4">
                            <h1 class="text-5xl font-bold mb-4">{APP_TITLE}</h1>
                            <h2 class="text-4xl font-bold">"Please Login"</h2>
                        </div>
                        <form on:submit=on_submit>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">"API Server"</span></label>
                                <BaseUrlInput value=base_url.get() new_base_url=set_base_url />
                            </div>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">"Username"</span></label>
                                <input
                                    prop:value={move || username.get()}
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
                                    prop:value={move || password.get()}
                                    on:input=move |ev| {set_password.set(event_target_value(&ev))}
                                    type="password"
                                    class="input input-bordered"
                                    placeholder="e.g. ••••••••"
                                    autocomplete="current-password"
                                    required=true
                                />
                            </div>
                            <div class="form-control btn-group btn-group-vertical">
                                {move || {
                                    view!{cx,
                                        <button
                                            class="btn btn-primary"
                                            // class="loading"
                                            class:loading=fetch_token.pending().get()
                                            type="submit"
                                            disabled=move || base_url.get().is_none()
                                        >
                                            "Login"
                                        </button>
                                    }
                                }}
                                <A href="/signup" class="btn">{"Signup Instead?"}</A>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
