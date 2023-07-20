use gloo::net::http::Request;
use mcc_frontend_types::{
    query::RecipesFilter, recipe, stats, user, ApiInfo, Login, LoginToken, StoredLogin,
};
use serde::de::DeserializeOwned;
use std::convert::From;

/// Sanitise given URL:
/// - Remove URLs ending with /
pub fn sanitise_base_url(base: String) -> String {
    let base = match base.strip_suffix('/') {
        Some(v) => v.to_owned(),
        None => base,
    };
    base
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiInternalError {
    Connection,
    Deserialization,
    Generic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiResponseError {
    pub status_code: u16,
}

/// When something goes wrong with a web request
#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    /// Something wrong with client
    Internal(ApiInternalError),
    /// Something wrong with the received response
    Response(ApiResponseError),
}

impl ApiError {
    /// Handle response errors
    pub fn from_response_result(
        response: Result<gloo::net::http::Response, gloo::net::Error>,
    ) -> Result<gloo::net::http::Response, Self> {
        match response {
            Ok(v) => Ok(v),
            Err(err) => match err {
                gloo::net::Error::JsError(_) => {
                    Err(ApiError::Internal(ApiInternalError::Connection))
                }
                _ => Err(ApiError::Internal(ApiInternalError::Generic)),
            },
        }
    }

    /// Handle internal errors,
    /// validating the received JSON matches given type
    pub async fn check_json_response_ok<T>(response: gloo::net::http::Response) -> Result<T, Self>
    where
        T: DeserializeOwned,
    {
        match response.ok() {
            false => Err(ApiError::Response(ApiResponseError {
                status_code: response.status(),
            })),
            true => match response.json::<T>().await {
                Err(err) => match err {
                    gloo::net::Error::SerdeError(_) => {
                        Err(ApiError::Internal(ApiInternalError::Deserialization))
                    }
                    _ => Err(ApiError::Internal(ApiInternalError::Generic)),
                },
                Ok(v) => Ok(v),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Api {
    base_url: String,
    login_token: Option<LoginToken>,
}

impl Api {
    pub fn new(base: String, token: Option<LoginToken>) -> Self {
        Api {
            base_url: sanitise_base_url(base),
            login_token: token,
        }
    }

    fn get_authorization_value(&self) -> Option<String> {
        self.login_token
            .as_ref()
            .map(|token| format!("{} {}", token.r#type, token.token))
    }

    pub async fn get_api_info(&self) -> Result<ApiInfo, ApiError> {
        let req_url = format!("{}/api/info/", self.base_url);
        let response = ApiError::from_response_result(Request::get(&req_url).send().await)?;
        ApiError::check_json_response_ok::<ApiInfo>(response).await
    }

    pub async fn post_login(&self, login: &Login) -> Result<LoginToken, ApiError> {
        let req_url = format!("{}/login/", self.base_url);
        let response = ApiError::from_response_result(
            Request::post(&req_url).json(login).unwrap().send().await,
        )?;
        ApiError::check_json_response_ok::<LoginToken>(response).await
    }

    pub async fn post_create_account(
        &self,
        details: &user::CreateUser,
    ) -> Result<user::User, ApiError> {
        let req_url = format!("{}/users/", self.base_url);
        let response = ApiError::from_response_result(
            Request::post(&req_url).json(details).unwrap().send().await,
        )?;
        ApiError::check_json_response_ok::<user::User>(response).await
    }

    pub async fn get_labels(&self) -> Result<Vec<String>, ApiError> {
        let req_url = format!("{}/labels/", self.base_url);
        let response = ApiError::from_response_result(
            Request::get(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .send()
                .await,
        )?;
        ApiError::check_json_response_ok::<Vec<String>>(response).await
    }

    pub async fn get_recipes(
        &self,
        filters: &RecipesFilter,
    ) -> Result<Vec<recipe::Recipe>, ApiError> {
        let req_url = format!(
            "{}/recipes/?{}",
            self.base_url,
            serde_url_params::to_string(&filters).unwrap(),
        );
        let response = ApiError::from_response_result(
            Request::get(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .send()
                .await,
        )?;
        ApiError::check_json_response_ok::<Vec<recipe::Recipe>>(response).await
    }

    pub async fn get_recipe_by_id(&self, id: String) -> Result<recipe::Recipe, ApiError> {
        let req_url = format!("{}/recipes/{}/", self.base_url, &id);
        let response = ApiError::from_response_result(
            Request::get(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .send()
                .await,
        )?;
        ApiError::check_json_response_ok::<recipe::Recipe>(response).await
    }

    pub async fn get_stats(&self) -> Result<stats::AccountStats, ApiError> {
        let req_url = format!("{}/stats/me/", self.base_url);
        let response = ApiError::from_response_result(
            Request::get(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .send()
                .await,
        )?;
        ApiError::check_json_response_ok::<stats::AccountStats>(response).await
    }

    pub async fn post_new_recipe(
        &self,
        new_recipe: &recipe::CreateRecipe,
    ) -> Result<recipe::Recipe, ApiError> {
        let req_url = format!("{}/recipes/", self.base_url);
        let response = ApiError::from_response_result(
            Request::post(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .json(new_recipe)
                .unwrap()
                .send()
                .await,
        )?;
        ApiError::check_json_response_ok::<recipe::Recipe>(response).await
    }

    pub async fn patch_update_recipe(
        &self,
        id: String,
        updated_recipe: &recipe::UpdateRecipe,
    ) -> Result<(), ApiError> {
        let req_url = format!("{}/recipes/{}/", self.base_url, id);
        ApiError::from_response_result(
            Request::patch(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .json(updated_recipe)
                .unwrap()
                .send()
                .await,
        )?;
        Ok(())
    }

    pub async fn delete_recipe(&self, id: &str) -> Result<(), ApiError> {
        let req_url = format!("{}/recipes/{}/", self.base_url, id);
        ApiError::from_response_result(
            Request::delete(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .send()
                .await,
        )?;
        Ok(())
    }

    pub async fn post_recipe_image(
        &self,
        id: String,
        file: web_sys::File,
    ) -> Result<String, ApiError> {
        let req_url = format!("{}/recipes/{}/image/", self.base_url, id);
        let response = ApiError::from_response_result(
            Request::post(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .body(file)
                .map_err(|_| ApiError::Internal(ApiInternalError::Generic))?
                .send()
                .await,
        )?;
        ApiError::check_json_response_ok::<String>(response).await
    }

    pub async fn delete_recipe_image(&self, id: String) -> Result<(), ApiError> {
        let req_url = format!("{}/recipes/{}/image/", self.base_url, id);
        ApiError::from_response_result(
            Request::delete(&req_url)
                .header("Authorization", &self.get_authorization_value().unwrap())
                .send()
                .await,
        )?;
        Ok(())
    }
}

impl From<StoredLogin> for Api {
    fn from(login: StoredLogin) -> Self {
        Api {
            base_url: login.api_url,
            login_token: Some(login.token),
        }
    }
}
