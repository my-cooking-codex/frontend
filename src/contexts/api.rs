use leptos::*;
use mcc_frontend_core::api::Api;

use super::login::{use_login, CurrentLogin};

#[derive(Copy, Clone)]
pub struct CurrentApi {
    pub api: Signal<Option<Api>>,
}

impl CurrentApi {
    pub fn new() -> Self {
        let CurrentLogin { login, .. } = use_login();

        let api = move || {
            let login = login.get();
            login.map(|v| Api::from(v.clone()))
        };

        Self {
            api: Signal::derive(api),
        }
    }
}

pub fn use_api() -> CurrentApi {
    use_context::<CurrentApi>().expect("unable to get current api context")
}
