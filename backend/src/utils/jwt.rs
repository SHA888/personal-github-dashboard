use crate::error::AppError;
use crate::models::jwt::Claims; // Use the new Claims struct from models
use crate::utils::config::Config; // Import Config for JWT secret
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

pub fn create_jwt(
    sub: &str,
    roles: Vec<String>,
    config: &Config,
    expires_in: usize, // e.g., 3600 for 1 hour
) -> Result<String, jsonwebtoken::errors::Error> {
    let iat = Utc::now().timestamp() as usize;
    let expiration = (Utc::now() + Duration::seconds(expires_in as i64)).timestamp() as usize;

    let claims = Claims {
        sub: sub.to_owned(),
        iat,
        exp: expiration,
        roles,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
}

pub fn validate_jwt(token: &str, config: &Config) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
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
        jsonwebtoken::errors::ErrorKind::InvalidAudience => {
            AppError::Unauthorized("Invalid audience".to_string())
        }
        jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
            AppError::Unauthorized("Invalid issuer".to_string())
        }
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
    // Config is available via super::* as crate::utils::config::Config

    // Helper function to load config for tests
    fn get_test_config() -> Config {
        dotenv::dotenv().ok(); // Ensure .env is loaded, especially for tests
        Config::from_env() // Relies on test fallbacks in Config::from_env if full .env isn't present
    }

    #[test]
    fn test_jwt_creation_and_validation() {
        let config = get_test_config();
        let user_id = Uuid::new_v4().to_string();
        let user_roles = vec!["user".to_string(), "editor".to_string()];
        let expires_in_seconds = 3600; // 1 hour

        let token = create_jwt(&user_id, user_roles.clone(), &config, expires_in_seconds)
            .expect("Failed to create token");
        assert!(!token.is_empty());

        let token_data = validate_jwt(&token, &config).expect("Failed to validate token");

        assert_eq!(token_data.claims.sub, user_id);
        assert_eq!(token_data.claims.roles, user_roles);

        let now_ts = Utc::now().timestamp() as usize;
        assert!(token_data.claims.iat <= now_ts);
        assert!(token_data.claims.exp > now_ts);
        // Allow a small delta for the exact expiration check due to execution time between iat and exp calculation
        assert!(
            (token_data.claims.exp.saturating_sub(token_data.claims.iat)) >= expires_in_seconds
        );
        assert!(
            (token_data.claims.exp.saturating_sub(token_data.claims.iat)) <= expires_in_seconds + 1
        ); // Allow 1s delta
    }

    #[test]
    fn test_invalid_token_format() {
        let config = get_test_config();
        let invalid_token_string = "this.is.not.a.valid.jwt";

        let result = validate_jwt(invalid_token_string, &config);
        assert!(result.is_err());
        match result.err().unwrap() {
            AppError::Unauthorized(msg) => {
                assert!(msg.contains("Invalid token") || msg.contains("JWT validation error"))
            }
            _ => panic!("Expected Unauthorized error for malformed token"),
        }
    }

    #[test]
    fn test_invalid_signature() {
        let config_for_validation = get_test_config();
        let user_id = Uuid::new_v4().to_string();
        let user_roles = vec!["test_role".to_string()];
        let expires_in = 3600;

        // Create a config with a different secret for signing
        let mut config_for_signing = Config::from_env(); // Base it on env or defaults
        config_for_signing.jwt_secret = "a_totally_different_secret_for_signing".to_string();

        let token_signed_with_wrong_secret = create_jwt(
            &user_id,
            user_roles.clone(),
            &config_for_signing,
            expires_in,
        )
        .expect("Token encoding with custom secret should succeed");

        let result = validate_jwt(&token_signed_with_wrong_secret, &config_for_validation);
        assert!(result.is_err());
        match result.err().unwrap() {
            AppError::Unauthorized(msg) => {
                assert!(msg.contains("Invalid signature") || msg.contains("Invalid token"))
            }
            _ => panic!("Expected Unauthorized/InvalidSignature error"),
        }
    }

    #[test]
    fn test_expired_token() {
        let config = get_test_config();
        let user_id = Uuid::new_v4().to_string();
        let user_roles = vec!["user".to_string()];

        let iat_past = (Utc::now() - Duration::hours(2)).timestamp() as usize;
        let expiration_past = (Utc::now() - Duration::hours(1)).timestamp() as usize;

        let expired_claims_data = Claims {
            sub: user_id,
            iat: iat_past,
            exp: expiration_past,
            roles: user_roles,
        };

        let expired_token = encode(
            &Header::default(),
            &expired_claims_data,
            &EncodingKey::from_secret(config.jwt_secret.as_ref()),
        )
        .expect("Encoding expired token failed");

        let result = validate_jwt(&expired_token, &config);
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
