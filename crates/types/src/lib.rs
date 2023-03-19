use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod query;
pub mod recipe;
pub mod stats;
pub mod user;

use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoginToken {
    pub r#type: String,
    pub token: String,
    pub expiry: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoredLogin {
    pub api_url: String,
    pub media_url: String,
    pub token: LoginToken,
}

/// A fraction type that can be converted to f32 and f64.
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl From<f32> for Fraction {
    fn from(f: f32) -> Self {
        // check if the float is already a whole number
        if f.fract() == 0.0 {
            return Self::new(f as i32, 1);
        }
        // otherwise, find the closest fraction
        // FIXME will not work for all cases
        let mut numerator = 1;
        let mut denominator = 1;
        while (numerator as f32 / denominator as f32 - f).abs() > 0.0001 {
            denominator += 1;
            numerator = (f * denominator as f32).round() as i32;
        }
        Self::new(numerator, denominator)
    }
}

impl FromStr for Fraction {
    type Err = ();

    /// Parse a fraction from a string.
    /// The string must be in the format of `numerator/denominator`.
    /// FIXME make this more robust (remove unwrap usage and return actual errors)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('/').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let numerator = parts[0].parse().unwrap();
        let denominator = parts[1].parse().unwrap();
        Ok(Fraction {
            numerator,
            denominator,
        })
    }
}

impl From<Fraction> for f32 {
    fn from(val: Fraction) -> Self {
        val.numerator as f32 / val.denominator as f32
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // if the fraction is a whole number, just return the numerator
        if self.denominator == 1 {
            return write!(f, "{}", self.numerator);
        }
        // otherwise, return the fraction
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
