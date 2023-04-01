use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod query;
pub mod recipe;
pub mod stats;
pub mod user;

use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    pub api_version_major: usize,
    pub api_version_minor: usize,
    pub registration_allowed: bool,
}

/// A fraction type that can be converted to f32.
///
/// Handles fractions with a whole number part, e.g. `1 1/2`.
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
    /// The string must be in the format of:
    /// - (fraction) `numerator/denominator`
    /// - (mixed number fraction) `whole numerator/denominator`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_parts(parts: &str) -> Result<(i32, i32), ()> {
            let (numerator, denominator) = match parts.split_once('/') {
                Some((numerator, denominator)) => (numerator, denominator),
                None => return Err(()),
            };
            let numerator = numerator.parse().map_err(|_| ())?;
            let denominator = denominator.parse().map_err(|_| ())?;
            Ok((numerator, denominator))
        }
        let (whole, (mut numerator, denominator)): (Option<i32>, (i32, i32)) =
            match s.split_once(' ') {
                Some((whole, parts)) => (Some(whole.parse().map_err(|_| ())?), parse_parts(parts)?),
                None => (None, parse_parts(s)?),
            };

        if let Some(whole) = whole {
            numerator = whole * denominator + numerator;
        }

        Ok(Self {
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
        if self.numerator > self.denominator {
            // mixed number fraction
            let whole = self.numerator / self.denominator;
            let remainder = self.numerator % self.denominator;
            write!(f, "{} {}/{}", whole, remainder, self.denominator)
        } else {
            // fraction, just return it
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

// TODO this could be made into a separate external crate
/// A struct that represents a second duration in hours, minutes, and seconds.
#[derive(Debug, Default, Clone, Copy)]
pub struct HourMinuteSecond {
    pub hours: usize,
    pub minutes: usize,
    pub seconds: usize,
}

impl HourMinuteSecond {
    pub fn new(hours: usize, minutes: usize, seconds: usize) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    /// Construct from seconds.
    pub fn from_secs(seconds: usize) -> Self {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;
        Self::new(hours, minutes, seconds)
    }

    /// Convert into seconds.
    pub fn as_secs(&self) -> usize {
        (self.hours * 3600) + (self.minutes * 60) + self.seconds
    }

    pub fn as_hms(&self) -> String {
        format!("{}h {}m {}s", self.hours, self.minutes, self.seconds)
    }

    /// Simplify the current values to smallest possible.
    ///
    /// e.g. (0 hours, 70 minutes, 0 second) -> (1 hour, 10 minutes, 0 second)
    pub fn simplify(&self) -> Self {
        let seconds = self.as_secs();
        Self::from_secs(seconds)
    }
}

impl PartialEq for HourMinuteSecond {
    fn eq(&self, other: &Self) -> bool {
        self.as_secs() == other.as_secs()
    }
}

#[cfg(test)]
mod tests_hour_minute_second {
    use super::*;

    #[test]
    fn test_from_secs() {
        let hms = HourMinuteSecond::from_secs(3661);
        assert_eq!(hms.hours, 1);
        assert_eq!(hms.minutes, 1);
        assert_eq!(hms.seconds, 1);

        let hms = HourMinuteSecond::from_secs(15600);
        assert_eq!(hms.hours, 4);
        assert_eq!(hms.minutes, 20);
        assert_eq!(hms.seconds, 0);

        let hms = HourMinuteSecond::from_secs(7200);
        assert_eq!(hms.hours, 2);
        assert_eq!(hms.minutes, 0);
        assert_eq!(hms.seconds, 0);

        let hms = HourMinuteSecond::from_secs(4200);
        assert_eq!(hms.hours, 1);
        assert_eq!(hms.minutes, 10);
        assert_eq!(hms.seconds, 0);
    }

    #[test]
    fn test_as_secs() {
        let hms = HourMinuteSecond::new(1, 1, 1);
        assert_eq!(hms.as_secs(), 3661);

        let hms = HourMinuteSecond::new(4, 20, 0);
        assert_eq!(hms.as_secs(), 15600);

        let hms = HourMinuteSecond::new(2, 0, 0);
        assert_eq!(hms.as_secs(), 7200);

        let hms = HourMinuteSecond::new(1, 10, 0);
        assert_eq!(hms.as_secs(), 4200);
    }

    #[test]
    fn test_simplify() {
        let hms = HourMinuteSecond::new(0, 70, 0).simplify();
        assert_eq!(hms.hours, 1);
        assert_eq!(hms.minutes, 10);
        assert_eq!(hms.seconds, 0);

        let hms = HourMinuteSecond::new(0, 0, 3661).simplify();
        assert_eq!(hms.hours, 1);
        assert_eq!(hms.minutes, 1);
        assert_eq!(hms.seconds, 1);

        let hms = HourMinuteSecond::new(4, 20, 0).simplify();
        assert_eq!(hms.hours, 4);
        assert_eq!(hms.minutes, 20);
        assert_eq!(hms.seconds, 0);
    }
}

#[cfg(test)]
mod tests_fraction {
    use super::*;

    #[test]
    fn test_from_str() {
        let fraction = Fraction::from_str("1/2").unwrap();
        assert_eq!(fraction.numerator, 1);
        assert_eq!(fraction.denominator, 2);

        let fraction = Fraction::from_str("1 1/2").unwrap();
        assert_eq!(fraction.numerator, 3);
        assert_eq!(fraction.denominator, 2);

        let fraction = Fraction::from_str("1 1/4").unwrap();
        assert_eq!(fraction.numerator, 5);
        assert_eq!(fraction.denominator, 4);
    }

    #[test]
    fn test_to_str() {
        let fraction = Fraction::new(1, 2);
        assert_eq!(fraction.to_string(), "1/2");

        let fraction = Fraction::new(1, 3);
        assert_eq!(fraction.to_string(), "1/3");

        let fraction = Fraction::new(7, 3);
        assert_eq!(fraction.to_string(), "2 1/3");
    }

    #[test]
    fn from_f32() {
        let fraction = Fraction::from(0.5f32);
        assert_eq!(fraction.numerator, 1);
        assert_eq!(fraction.denominator, 2);

        let fraction = Fraction::from(0.3333f32);
        assert_eq!(fraction.numerator, 1);
        assert_eq!(fraction.denominator, 3);

        let fraction = Fraction::from(2.3333f32);
        assert_eq!(fraction.numerator, 7);
        assert_eq!(fraction.denominator, 3);
    }
}
