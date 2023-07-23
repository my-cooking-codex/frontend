use chrono::{DateTime, Days, Utc};
use serde::{Deserialize, Serialize};

use crate::HumanDateFormats;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub name: String,
    pub owner_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub name: String,
    pub location_id: String,
    pub quantity: isize,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub expiry: Option<DateTime<Utc>>,
    #[serde(default)]
    pub labels: Vec<String>,
}

impl Item {
    pub fn is_expired(&self) -> bool {
        match self.expiry {
            Some(v) => Utc::now() >= v,
            None => false,
        }
    }

    pub fn is_expired_with_offset(&self, days: u64) -> bool {
        match self.expiry {
            Some(v) => {
                Utc::now()
                    .checked_add_days(Days::new(days))
                    .unwrap_or_else(Utc::now)
                    >= v
            }
            None => false,
        }
    }

    pub fn expiry_to_human(&self, fmt: &HumanDateFormats) -> Option<String> {
        let fmt = match fmt {
            HumanDateFormats::YearMonthDay => super::YEAR_MONTH_DAY_FMT,
            HumanDateFormats::DayMonthYear => super::DAY_MONTH_YEAR_FMT,
            HumanDateFormats::MonthDayYear => super::MONTH_DAY_YEAR_FMT,
        };
        self.expiry.map(|v| v.naive_local().format(fmt).to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateLocation {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateItem {
    pub name: String,
    pub quantity: isize,
    pub notes: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub labels: Vec<String>,
}

pub type UpdateLocation = CreateLocation;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItem {
    pub name: String,
    pub location_id: String,
    pub quantity: isize,
    pub notes: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub labels: Vec<String>,
}
