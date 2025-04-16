#![allow(dead_code)] // Allow dead code for now
use chrono::{DateTime, Utc};

pub fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

pub fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}
