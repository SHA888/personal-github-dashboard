use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use ring::aead;
use ring::aead::{Aad, BoundKey, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};

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

impl OAuthToken {
    pub fn encrypt_token(plain: &str, key: &[u8; 32]) -> Result<Vec<u8>, String> {
        let rng = SystemRandom::new();
        let mut nonce = [0u8; NONCE_LEN];
        rng.fill(&mut nonce)
            .map_err(|e| format!("nonce gen: {:?}", e))?;
        let nonce_bytes = nonce;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, key).map_err(|e| format!("key: {:?}", e))?;
        let mut sealing_key = LessSafeKey::new(unbound_key);
        let mut in_out = plain.as_bytes().to_vec();
        sealing_key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| format!("seal: {:?}", e))?;
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);
        Ok(result)
    }
    pub fn decrypt_token(ciphertext: &[u8], key: &[u8; 32]) -> Result<String, String> {
        if ciphertext.len() < NONCE_LEN {
            return Err("ciphertext too short".into());
        }
        let (nonce_bytes, ct) = ciphertext.split_at(NONCE_LEN);
        let nonce = Nonce::try_assume_unique_for_key(nonce_bytes).map_err(|_| "invalid nonce")?;
        let unbound_key =
            UnboundKey::new(&AES_256_GCM, key).map_err(|e| format!("key: {:?}", e))?;
        let mut opening_key = LessSafeKey::new(unbound_key);
        let mut in_out = ct.to_vec();
        let plain = opening_key
            .open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| format!("open: {:?}", e))?;
        String::from_utf8(plain.to_vec()).map_err(|e| format!("utf8: {:?}", e))
    }
}
