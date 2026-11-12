//! Time model (D-21): UTC instants; calendar dates are Europe/Berlin unless noted.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Instant stored/compared in UTC.
pub type UtcInstant = DateTime<Utc>;

/// Fachliches Kalenderdatum (ISO date, interpreted in Europe/Berlin for reporting).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CalendarDate(pub NaiveDate);

impl CalendarDate {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(Self)
    }
}
