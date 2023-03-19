use leptos::*;
use mcc_frontend_core::api::Api;

use super::login::{use_login, CurrentLogin};

#[derive(Copy, Clone)]
pub struct CurrentApi {
    pub api: ReadSignal<Option<Api>>,
    pub set_api: WriteSignal<Option<Api>>,
}

impl CurrentApi {
    pub fn new(cx: Scope) -> Self {
        let CurrentLogin { login, .. } = use_login(cx);
        let (api, set_api) = create_signal(cx, login.get().as_ref().map(|v| Api::from(v.clone())));

        // XXX this is not recommended
        create_effect(cx, move |_| {
            match login.get() {
                Some(v) => set_api.set(Some(Api::from(v))),
                None => set_api.set(None),
            };
        });

        Self { api, set_api }
    }

    /// get the current `Api`, will panic if value is none.
    pub fn must_get(&self) -> Api {
        self.api.get().expect("unable to get current api, it is none")
    }
}

pub fn use_api(cx: Scope) -> CurrentApi {
    use_context::<CurrentApi>(cx).expect("unable to get current api context")
}
