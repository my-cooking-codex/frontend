use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngredient {
    pub name: String,
    pub amount: f32,
    pub unit_type: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStep {
    #[serde(default)]
    pub title: Option<String>,
    pub description: String,
}

pub type CreateInfo = Info;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecipe {
    pub title: String,
    #[serde(default)]
    pub info: CreateInfo,
    #[serde(default)]
    pub short_description: Option<String>,
    #[serde(default)]
    pub long_description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ingredients: Vec<CreateIngredient>,
    #[serde(default)]
    pub steps: Vec<CreateStep>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIngredient {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub amount: Option<f32>,
    #[serde(default)]
    pub unit_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStep {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

pub type UpdateInfo = Info;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecipe {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub info: UpdateInfo,
    #[serde(default)]
    pub short_description: Option<String>,
    #[serde(default)]
    pub long_description: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub ingredients: Option<Vec<UpdateIngredient>>,
    #[serde(default)]
    pub steps: Option<Vec<UpdateStep>>,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit_type: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    #[serde(default)]
    pub title: Option<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InfoYields {
    pub value: usize,
    pub unit_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub yields: Option<InfoYields>,
    pub cook_time: usize,
    pub prep_time: usize,
    pub freezable: bool,
    pub microwave_only: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    #[serde(default)]
    pub info: Info,
    #[serde(default)]
    pub short_description: Option<String>,
    #[serde(default)]
    pub long_description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ingredients: Vec<Ingredient>,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}
