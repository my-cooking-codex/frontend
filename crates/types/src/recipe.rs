use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngredient {
    pub name: String,
    pub amount: f32,
    pub unit_type: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateStep {
    pub title: Option<String>,
    pub description: String,
}

pub type CreateInfo = Info;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecipe {
    pub title: String,
    pub info: CreateInfo,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub tags: Vec<String>,
    pub ingredients: Vec<CreateIngredient>,
    pub steps: Vec<CreateStep>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub amount: Option<f32>,
    pub unit_type: Option<String>,
    pub description: Option<String>,
}

impl From<Ingredient> for UpdateIngredient {
    fn from(ingredient: Ingredient) -> Self {
        Self {
            name: Some(ingredient.name),
            amount: Some(ingredient.amount),
            unit_type: Some(ingredient.unit_type),
            description: ingredient.description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStep {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl From<Step> for UpdateStep {
    fn from(step: Step) -> Self {
        Self {
            title: step.title,
            description: Some(step.description),
        }
    }
}

pub type UpdateInfo = Info;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecipe {
    pub title: Option<String>,
    pub info: UpdateInfo,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ingredients: Option<Vec<UpdateIngredient>>,
    pub steps: Option<Vec<UpdateStep>>,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub name: String,
    pub amount: f32,
    pub unit_type: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Step {
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
    #[serde(default)]
    pub cook_time: usize,
    #[serde(default)]
    pub prep_time: usize,
    #[serde(default)]
    pub freezable: bool,
    #[serde(default)]
    pub microwave_only: bool,
    pub source: Option<String>,
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
}
