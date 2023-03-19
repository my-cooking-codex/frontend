use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipesFilter {
    pub page: usize,
    pub per_page: usize,
}

impl Default for RecipesFilter {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}
