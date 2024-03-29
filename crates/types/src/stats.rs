use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountStats {
    pub user_count: usize,
    pub recipe_count: usize,
    pub pantry_item_count: usize,
    pub label_count: usize,
}
