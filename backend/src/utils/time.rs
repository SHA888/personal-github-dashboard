use chrono::{DateTime, TimeZone, Utc};

pub fn format_timestamp(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    Utc.datetime_from_str(s, "%Y-%m-%d %H:%M:%S UTC").ok()
}
