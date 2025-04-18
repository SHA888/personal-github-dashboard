use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use time::{Duration, OffsetDateTime};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = OffsetDateTime::now_utc();
        let exp = now + Duration::days(7);

        Self {
            sub: user_id,
            exp: exp.unix_timestamp(),
            iat: now.unix_timestamp(),
        }
    }
}

pub fn create_token(user_id: Uuid, secret: &[u8]) -> Result<String, AppError> {
    let claims = Claims::new(user_id);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| AppError::InternalError(e.to_string()))
}

#[allow(dead_code)]
pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(e.to_string()))
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, AppError> {
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret);
    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;
    Ok(token_data.claims)
}

#[allow(dead_code)]
pub fn parse_duration(duration_str: &str) -> Result<Duration, AppError> {
    let num: i64 = duration_str
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .map_err(|_| AppError::InternalError("Invalid duration format".to_string()))?;

    let unit = duration_str
        .chars()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>();

    match unit.as_str() {
        "s" | "sec" | "second" | "seconds" => Ok(Duration::seconds(num)),
        "m" | "min" | "minute" | "minutes" => Ok(Duration::minutes(num)),
        "h" | "hr" | "hour" | "hours" => Ok(Duration::hours(num)),
        "d" | "day" | "days" => Ok(Duration::days(num)),
        "w" | "week" | "weeks" => Ok(Duration::days(num * 7)),
        _ => Err(AppError::InternalError("Invalid duration unit".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";

        // Create token
        let token = create_token(user_id, secret.as_bytes()).expect("Failed to create token");
        assert!(!token.is_empty());

        // Validate token
        let claims = verify_token(&token, secret.as_bytes()).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);

        // Check timestamps (allow some leeway)
        let now = OffsetDateTime::now_utc().unix_timestamp();
        assert!(claims.iat <= now);
        assert!(claims.exp > claims.iat);
        assert!(claims.exp > now);
    }

    #[test]
    fn test_invalid_token() {
        let secret = "test_secret_key";
        let invalid_token = "this.is.not.a.valid.token";

        let result = verify_token(invalid_token, secret.as_bytes());
        assert!(result.is_err());
        match result {
            Ok(_) => panic!("Expected error for invalid token"),
            Err(e) => match e {
                AppError::Unauthorized(_) => (), // Expected
                _ => panic!("Unexpected error type"),
            },
        }
    }

    #[test]
    fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";

        // Create claims with expired timestamp
        let now = OffsetDateTime::now_utc();
        let claims = Claims {
            sub: user_id,
            exp: (now - Duration::hours(1)).unix_timestamp(),
            iat: (now - Duration::hours(2)).unix_timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        let result = verify_token(&token, secret.as_bytes());
        assert!(result.is_err());
        match result {
            Ok(_) => panic!("Expected error for expired token"),
            Err(e) => match e {
                AppError::Unauthorized(_) => (), // Expected
                _ => panic!("Unexpected error type"),
            },
        }
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("30s"), Ok(Duration::seconds(30)));
        assert_eq!(parse_duration("5m"), Ok(Duration::minutes(5)));
        assert_eq!(parse_duration("24h"), Ok(Duration::hours(24)));
        assert_eq!(parse_duration("7d"), Ok(Duration::days(7)));
        assert_eq!(parse_duration("2w"), Ok(Duration::days(14)));
        assert_eq!(
            parse_duration("invalid"),
            Err(AppError::InternalError(
                "Invalid duration format".to_string()
            ))
        );
    }
}
