use crate::error::AppError;
use chrono::{Duration, TimeZone, Utc}; // Use TimeZone for parsing duration string
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // Subject (user ID)
    pub exp: usize, // Expiration time (timestamp)
    pub iat: usize, // Issued at (timestamp)
}

// Function to create a JWT token
pub fn create_token(user_id: Uuid, secret: &str, expires_in_str: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;

    // Parse the expiration duration string (e.g., "24h", "7d")
    let expires_in_duration = parse_duration(expires_in_str).ok_or_else(|| {
        AppError::InternalError(format!("Invalid JWT_EXPIRES_IN format: {}", expires_in_str))
    })?;

    let expiration_time = now + expires_in_duration;
    let exp = expiration_time.timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::InternalError(format!("Failed to create JWT: {}", e)))
}

// Function to validate a JWT token and extract claims
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);
    // Custom validation logic can be added here if needed (e.g., checking issuer)

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
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
        _ => AppError::InternalError(format!("JWT validation error: {}", e)),
    })
}

// Helper function to parse duration strings like "24h", "7d", "30m"
fn parse_duration(duration_str: &str) -> Option<Duration> {
    let duration_str = duration_str.trim();
    let value_str = duration_str
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    let unit_str = duration_str
        .chars()
        .skip_while(|c| c.is_digit(10))
        .collect::<String>();

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
        let expires_in = "1h";

        // Create token
        let token = create_token(user_id, secret, expires_in).expect("Failed to create token");
        assert!(!token.is_empty());

        // Validate token
        let claims = validate_token(&token, secret).expect("Failed to validate token");
        assert_eq!(claims.sub, user_id);

        // Check timestamps (allow some leeway)
        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now);
        assert!(claims.exp > claims.iat);
        assert!(claims.exp > now);
    }

    #[test]
    fn test_invalid_token() {
        let secret = "test_secret_key";
        let invalid_token = "this.is.not.a.valid.token";

        let result = validate_token(invalid_token, secret);
        assert!(result.is_err());
        match result.err().unwrap() {
            AppError::Unauthorized(msg) => assert_eq!(msg, "Invalid token"),
            _ => panic!("Expected Unauthorized error for invalid token"),
        }
    }

    #[test]
    fn test_invalid_signature() {
        let user_id = Uuid::new_v4();
        let secret1 = "correct_secret";
        let secret2 = "wrong_secret";
        let expires_in = "1h";

        let token = create_token(user_id, secret1, expires_in).unwrap();
        let result = validate_token(&token, secret2);
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
        let expires_in = "-1h"; // Expired one hour ago

        // Need to manually construct claims because create_token might validate expiration internally
        let now = Utc::now();
        let iat = (now - Duration::try_hours(2).unwrap()).timestamp() as usize;
        let exp = (now - Duration::try_hours(1).unwrap()).timestamp() as usize;
        let claims = Claims {
            sub: user_id,
            exp,
            iat,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();

        let result = validate_token(&token, secret);
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
