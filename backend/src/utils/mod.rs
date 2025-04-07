use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: Utc::now(),
        }
    }
}
