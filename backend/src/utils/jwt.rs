use crate::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(
    sub: &str,
    secret: &str,
    expires_in: usize,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(expires_in as i64))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::Unauthorized("Token expired".to_string())
        }
        jsonwebtoken::errors::ErrorKind::InvalidToken => {
            AppError::Unauthorized("Invalid token".to_string())
        }
        jsonwebtoken::errors::ErrorKind::InvalidSignature => {
            AppError::Unauthorized("Invalid signature".to_string())
        }
        // Treat all other JWT errors as Unauthorized for security
        _ => AppError::Unauthorized(format!("JWT validation error: {}", e)),
    })
}

#[cfg(test)]
fn parse_duration(duration_str: &str) -> Option<Duration> {
    let duration_str = duration_str.trim();
    let value_str = duration_str
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let unit_str = duration_str
        .chars()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .trim()
        .to_string();

    let value = value_str.parse::<i64>().ok()?;

    match unit_str.as_str() {
        "s" => Duration::try_seconds(value),
        "m" => Duration::try_minutes(value),
        "h" => Duration::try_hours(value),
        "d" => Duration::try_days(value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_jwt_creation_and_validation() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";

        // Create token
        let token = create_jwt(&user_id.to_string(), secret, 3600).expect("Failed to create token");
        assert!(!token.is_empty());

        // Validate token
        let claims = validate_jwt(&token, secret).expect("Failed to validate token");
        assert_eq!(claims.claims.sub, user_id.to_string());

        // Check timestamps (allow some leeway)
        let now = Utc::now().timestamp() as usize;
        assert!(claims.claims.exp > now);
    }

    #[test]
    fn test_invalid_token() {
        let secret = "test_secret_key";
        let invalid_token = "this.is.not.a.valid.token";

        let result = validate_jwt(invalid_token, secret);
        assert!(result.is_err());
        match result.err().unwrap() {
            AppError::Unauthorized(_msg) => {}
            _ => panic!("Expected Unauthorized error for invalid token"),
        }
    }

    #[test]
    fn test_invalid_signature() {
        let user_id = Uuid::new_v4();
        let secret1 = "correct_secret";
        let secret2 = "wrong_secret";
        let expires_in = 3600;

        let token = create_jwt(&user_id.to_string(), secret1, expires_in).unwrap();
        let result = validate_jwt(&token, secret2);
        assert!(result.is_err());
        match result.err().unwrap() {
            AppError::Unauthorized(msg) => assert_eq!(msg, "Invalid signature"),
            _ => panic!("Expected Unauthorized error for invalid signature"),
        }
    }

    #[test]
    fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key";

        // Manually construct claims for an expired token
        let now = Utc::now();
        let exp = (now - Duration::try_hours(1).unwrap()).timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            exp,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();

        let result = validate_jwt(&token, secret);
        assert!(result.is_err());
        match result.err().unwrap() {
            AppError::Unauthorized(msg) => assert_eq!(msg, "Token expired"),
            _ => panic!("Expected Unauthorized error for expired token"),
        }
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1s"), Duration::try_seconds(1));
        assert_eq!(parse_duration("30m"), Duration::try_minutes(30));
        assert_eq!(parse_duration("24h"), Duration::try_hours(24));
        assert_eq!(parse_duration("7d"), Duration::try_days(7));
        assert_eq!(parse_duration(" 10 d "), Duration::try_days(10));
        assert_eq!(parse_duration("invalid"), None);
        assert_eq!(parse_duration("1y"), None); // Year not supported yet
    }
}
