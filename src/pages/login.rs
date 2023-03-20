use crate::{
    contexts::{
        login::CurrentLogin,
        prelude::{use_api, use_login, use_toasts, CurrentApi, Toast},
    },
    helpers::{login_redirect_effect, LoginState},
};
use leptos::{ev::SubmitEvent, *};
use leptos_router::{AProps, A};
use mcc_frontend_core::{
    api::{sanitise_base_url, Api},
    APP_TITLE,
};
use mcc_frontend_types::{Login, StoredLogin};

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let CurrentLogin { set_login, .. } = use_login(cx);
    let CurrentApi { set_api, .. } = use_api(cx);
    let toasts = use_toasts(cx);

    let (base_url, set_base_url) = create_signal(cx, String::default());
    let (username, set_username) = create_signal(cx, String::default());
    let (password, set_password) = create_signal(cx, String::default());
    let (login_details, set_login_details) = create_signal::<Option<Login>>(cx, Option::default());

    login_redirect_effect(cx, LoginState::Unauthenticated, "/");

    let token = create_resource(
        cx,
        move || {},
        move |_| async move {
            if let Some(details) = login_details.get() {
                let base_url = sanitise_base_url(base_url.get());
                let api_url = format!("{}/api", base_url);
                let api = Api::new(api_url.clone(), None);
                // request oauth token, with given details
                let token = match api.post_login(&details).await {
                    Ok(v) => Some(v),
                    Err(_) => {
                        toasts.push(Toast {
                            message: "Error logging in".to_owned(),
                        });
                        None
                    }
                };
                // if successful, set api and login
                if let Some(token) = &token {
                    set_api.set(Some(Api::new(api_url.clone(), Some(token.clone()))));
                    set_login.set(Some(StoredLogin {
                        api_url,
                        media_url: format!("{}/media", base_url),
                        token: token.clone(),
                    }));
                    log::debug!("login successful");
                }
                return token;
            }
            None
        },
    );

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_login_details.set(Some(Login {
            username: username.get(),
            password: password.get(),
        }));
        token.refetch();
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
                                <label class="label"><span class="label-text">"URL"</span></label>
                                <input
                                    prop:value={move || base_url.get()}
                                    on:input=move |ev| {set_base_url.set(event_target_value(&ev))}
                                    type="url"
                                    class="input input-bordered"
                                    placeholder="e.g. https://..."
                                    required=true
                                />
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
                                    if token.loading().get() {
                                        view!(cx, <button type="submit" class="btn loading" disabled=true>"Login"</button>)
                                    } else {
                                        view!(cx, <button type="submit" class="btn btn-primary">"Login"</button>)
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
