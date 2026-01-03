use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;

pub struct EncryptionService {
    cipher: Aes256Gcm,
}

impl EncryptionService {
    pub fn new(key: &[u8]) -> Result<Self, anyhow::Error> {
        if key.len() != 32 {
            return Err(anyhow::anyhow!("Encryption key must be 32 bytes"));
        }
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| anyhow::anyhow!("Failed to create cipher: {:?}", e))?;
        Ok(Self { cipher })
    }

    /// Encrypts data and returns: nonce (12 bytes) + ciphertext
    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, anyhow::Error> {
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypts data where input is: nonce (12 bytes) + ciphertext
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<String, anyhow::Error> {
        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Invalid encrypted data"));
        }

        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in decrypted data: {}", e))
    }

    /// Helper to encrypt credentials for database connections
    pub fn encrypt_credentials(&self, password: &str) -> Result<String, anyhow::Error> {
        let encrypted = self.encrypt(password)?;
        Ok(hex::encode(encrypted))
    }

    /// Helper to decrypt credentials
    pub fn decrypt_credentials(&self, encrypted_hex: &str) -> Result<String, anyhow::Error> {
        let encrypted = hex::decode(encrypted_hex)
            .map_err(|e| anyhow::anyhow!("Invalid hex format: {}", e))?;
        self.decrypt(&encrypted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let key = [0u8; 32];
        let service = EncryptionService::new(&key).unwrap();
        
        let plaintext = "my-secret-password";
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_credentials_encryption() {
        let key = [0u8; 32];
        let service = EncryptionService::new(&key).unwrap();
        
        let password = "SuperSecret123!";
        let encrypted_hex = service.encrypt_credentials(password).unwrap();
        let decrypted = service.decrypt_credentials(&encrypted_hex).unwrap();
        
        assert_eq!(password, decrypted);
    }
}
