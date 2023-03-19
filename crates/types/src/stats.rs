use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountStats {
    pub user_count: usize,
    pub recipe_count: usize,
}
