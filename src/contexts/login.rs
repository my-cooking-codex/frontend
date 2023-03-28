use leptos::*;
use mcc_frontend_core::storage;
use mcc_frontend_types::StoredLogin;

#[derive(Copy, Clone)]
pub struct CurrentLogin {
    pub login: ReadSignal<Option<StoredLogin>>,
    pub set_login: WriteSignal<Option<StoredLogin>>,
}

impl CurrentLogin {
    pub fn new(cx: Scope) -> Self {
        let (login, set_login) = create_signal(cx, storage::read_login_token());

        create_effect(cx, move |_| {
            let new_login = login.get();
            match new_login {
                Some(v) => storage::set_login_token(v),
                None => storage::remove_login_token(),
            };
        });

        Self { login, set_login }
    }
}

pub fn use_login(cx: Scope) -> CurrentLogin {
    use_context::<CurrentLogin>(cx).expect("unable to get current login context")
}
