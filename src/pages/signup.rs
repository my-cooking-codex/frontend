use crate::{
    components::input::{BaseUrlInput, BaseUrlInputProps},
    contexts::prelude::{use_toasts, Toast},
    helpers::{api_error_to_toast, login_redirect_effect, LoginState},
};
use leptos::{ev::SubmitEvent, leptos_dom::helpers::location, *};
use leptos_router::{use_navigate, AProps, A};
use mcc_frontend_core::{api::Api, APP_TITLE};
use mcc_frontend_types::user::CreateUser;

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let toasts = use_toasts(cx);

    let (base_url, set_base_url) = create_signal::<Option<String>>(cx, location().origin().ok());
    let (username, set_username) = create_signal(cx, String::default());
    let (password, set_password) = create_signal(cx, String::default());
    let (password_confirm, set_password_confirm) = create_signal(cx, String::default());
    let (error_tooltip, set_error_tooltip) = create_signal(cx, String::default());

    login_redirect_effect(cx, LoginState::Unauthenticated, "/");

    let create_account = create_action(cx, move |args: &(String, CreateUser)| {
        let navigator = use_navigate(cx);
        let (base_url, details) = args.to_owned();
        async move {
            let api_url = format!("{}/api", base_url);
            let api = Api::new(api_url.clone(), None);
            match api.post_create_account(&details).await {
                Ok(_) => {
                    toasts.push(Toast {
                        message: "Account Created".to_owned(),
                    });
                    navigator("/login", Default::default()).unwrap();
                }
                Err(err) => {
                    toasts.push(api_error_to_toast(&err, "creating account"));
                }
            };
        }
    });

    create_effect(cx, move |_| {
        if password.get() != password_confirm.get() {
            set_error_tooltip.set("Passwords do not match".to_owned());
        } else {
            set_error_tooltip.set(String::default());
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        if let Some(base_url) = base_url.get() {
            if password.get() == password_confirm.get() {
                create_account.dispatch((
                    base_url,
                    CreateUser {
                        username: username.get(),
                        password: password.get(),
                    },
                ));
            }
        }
    };

    view! {cx,
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                    <div class="card-body">
                        <div class="mb-4">
                            <h1 class="text-5xl font-bold mb-4">{APP_TITLE}</h1>
                            <h2 class="text-4xl font-bold">"Create Account"</h2>
                        </div>
                        <form on:submit=on_submit>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">"API Server"</span></label>
                                <BaseUrlInput value=base_url.get() new_base_url=set_base_url />
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
                                    maxlength=30
                                    pattern="[a-zA-Z0-9]+"
                                    required=true
                                />
                            </div>
                            <div class="form-control mb-2">
                                <label class="label"><span class="label-text">"Password"</span></label>
                                <input
                                    prop:value=move || password.get()
                                    on:input=move |ev| {set_password.set(event_target_value(&ev))}
                                    type="password"
                                    class="input input-bordered"
                                    placeholder="e.g. ••••••••"
                                    autocomplete="new-password"
                                    required=true
                                />
                            </div>
                            <div class="form-control mb-6">
                                <label class="label"><span class="label-text">"Password Confirm"</span></label>
                                <input
                                    prop:value=move || password_confirm.get()
                                    on:input=move |ev| {set_password_confirm.set(event_target_value(&ev))}
                                    type="password"
                                    placeholder="e.g. ••••••••"
                                    autocomplete="new-password"
                                    class="input input-bordered"
                                    required=true
                                />
                            </div>
                            <div class="form-control btn-group btn-group-vertical">
                                {move || {
                                    if create_account.pending().get() {
                                        view!{cx, <><button type="submit" class="btn loading" disabled=true>"Signup"</button></>}
                                    } else if error_tooltip.get().is_empty() {
                                        view!{cx, <><button type="submit" class="btn btn-primary">"Signup"</button></>}
                                    } else {
                                        view!{cx,
                                            <><div class="tooltip tooltip-open tooltip-error" data-tip=move || error_tooltip.get()>
                                                <button type="submit" class="btn btn-disabled btn-block" disabled=true>"Signup"</button>
                                            </div></>
                                        }
                                    }
                                }}
                                <A href="/login" class="btn">"Login Instead?"</A>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
