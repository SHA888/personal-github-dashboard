use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use ring::aead::{AES_256_GCM, Aad, LessSafeKey, Nonce, UnboundKey};
use ring::rand::{SecureRandom, SystemRandom};

use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct OAuthToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub access_token: Vec<u8>,          // encrypted
    pub refresh_token: Option<Vec<u8>>, // encrypted
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("ciphertext too short")]
    CiphertextTooShort,
    #[error("invalid nonce")]
    InvalidNonce,
    #[error("key error: {0}")]
    Key(String),
    #[error("seal error: {0}")]
    Seal(String),
    #[error("open error: {0}")]
    Open(String),
    #[error("utf8 error: {0}")]
    Utf8(String),
    #[error("nonce generation error: {0}")]
    NonceGen(String),
}

impl OAuthToken {
    /// Encrypts a plaintext string using AES-256-GCM with a random nonce.
    ///
    /// The resulting byte vector contains the nonce followed by the ciphertext and authentication tag. Returns an error string if encryption fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let key = [0u8; 32];
    /// let plaintext = "my_secret_token";
    /// let encrypted = OAuthToken::encrypt_token(plaintext, &key).unwrap();
    /// assert!(encrypted.len() > 12); // Nonce + ciphertext
    /// ```
    pub fn encrypt_token(plain: &str, key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
        let rng = SystemRandom::new();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce)
            .map_err(|e| CryptoError::NonceGen(format!("{:?}", e)))?;
        let nonce_bytes = nonce;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, key).map_err(|e| CryptoError::Key(format!("{:?}", e)))?;
        let sealing_key = LessSafeKey::new(unbound_key);
        let mut in_out = plain.as_bytes().to_vec();
        sealing_key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| CryptoError::Seal(format!("{:?}", e)))?;
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);
        Ok(result)
    }
    /// Decrypts an AES-256-GCM encrypted OAuth token and returns the plaintext string.
    ///
    /// Expects the input to contain the nonce followed by the ciphertext, as produced by `encrypt_token`.
    /// Returns an error if the ciphertext is too short, the nonce or key is invalid, decryption fails, or the result is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// let key = [0u8; 32];
    /// let plaintext = "my_secret_token";
    /// let ciphertext = OAuthToken::encrypt_token(plaintext, &key).unwrap();
    /// let decrypted = OAuthToken::decrypt_token(&ciphertext, &key).unwrap();
    /// assert_eq!(decrypted, plaintext);
    /// ```
    pub fn decrypt_token(ciphertext: &[u8], key: &[u8; 32]) -> Result<String, CryptoError> {
        if ciphertext.len() < 12 {
            return Err(CryptoError::CiphertextTooShort);
        }
        let (nonce_bytes, ct) = ciphertext.split_at(12);
        let nonce =
            Nonce::try_assume_unique_for_key(nonce_bytes).map_err(|_| CryptoError::InvalidNonce)?;
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, key).map_err(|e| CryptoError::Key(format!("{:?}", e)))?;
        let opening_key = LessSafeKey::new(unbound_key);
        let mut in_out = ct.to_vec();
        let plain = opening_key
            .open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| CryptoError::Open(format!("{:?}", e)))?;
        String::from_utf8(plain.to_vec()).map_err(|e| CryptoError::Utf8(format!("{:?}", e)))
    }
}
