use gloo::storage::{LocalStorage, Storage};
use mcc_frontend_types::StoredLogin;

const LOGIN_DETAILS_KEY: &str = "login-details";

pub fn read_login_token() -> Option<StoredLogin> {
    LocalStorage::get::<StoredLogin>(LOGIN_DETAILS_KEY).ok()
}

pub fn set_login_token(login: StoredLogin) {
    LocalStorage::set(LOGIN_DETAILS_KEY, login).unwrap()
}

pub fn remove_login_token() {
    LocalStorage::delete(LOGIN_DETAILS_KEY)
}
