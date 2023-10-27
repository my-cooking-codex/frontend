use leptos::*;
use mcc_frontend_core::api::{ApiError, ApiInternalError};
use mcc_frontend_types::StoredLogin;

use crate::contexts::prelude::Toast;

/// Logout the user if the API returns a 401.
/// Returns true if the user was logged out
pub fn logout_on_401(set_login: &WriteSignal<Option<StoredLogin>>, error: &ApiError) -> bool {
    if let ApiError::Response(e) = error {
        if e.status_code == 401 {
            set_login.set(None);
            return true;
        }
    }
    false
}

/// Convert an API error to a toast message,
/// 'when' is added to the message to describe the action that failed
pub fn api_error_to_toast(error: &ApiError, when: &str) -> Toast {
    match error {
        ApiError::Internal(e) => match e {
            ApiInternalError::Connection => Toast {
                message: format!("Action failed as could not connect to server, when {when}"),
            },
            _ => {
                log::error!("Internal error handled: {e:?}, when {when}");
                Toast {
                    message: format!("Internal error occurred; action failed, when {when}"),
                }
            }
        },
        ApiError::Response(e) => match e.status_code {
            404 => Toast {
                message: format!("Action failed as resource was not found, when {when}"),
            },
            _ => Toast {
                message: format!(
                    "Action failed received status code '{}', when {when}",
                    e.status_code
                ),
            },
        },
    }
}
