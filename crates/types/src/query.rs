use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipesFilter {
    pub page: usize,
    pub per_page: usize,
    pub title: Option<String>,
    pub labels: Option<Vec<String>>,
    pub freezable: Option<bool>,
    pub microwave_only: Option<bool>,
}

impl Default for RecipesFilter {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            title: None,
            labels: None,
            freezable: None,
            microwave_only: None,
        }
    }
}
