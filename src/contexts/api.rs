use leptos::*;
use mcc_frontend_core::api::Api;

use super::login::{use_login, CurrentLogin};

#[derive(Copy, Clone)]
pub struct CurrentApi {
    pub api: Signal<Option<Api>>,
}

impl CurrentApi {
    pub fn new(cx: Scope) -> Self {
        let CurrentLogin { login, .. } = use_login(cx);

        let api = move || {
            let login = login.get();
            login.map(|v| Api::from(v.clone()))
        };

        Self {
            api: Signal::derive(cx, api),
        }
    }
}

pub fn use_api(cx: Scope) -> CurrentApi {
    use_context::<CurrentApi>(cx).expect("unable to get current api context")
}
