// Security and cryptographic communication module for CubeSat communications
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use log::{info, warn};
use rand::RngCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Telemetry,
    Command,
    Emergency,
    Admin,
}

#[derive(Debug, Clone)]
pub struct CryptoModule {
    encryption_key: Vec<u8>,
    signing_key: Vec<u8>,
    auth_tokens: HashMap<u32, (String, DateTime<Utc>)>,
    session_keys: HashMap<u32, Vec<u8>>,
}

impl CryptoModule {
    pub fn new() -> Self {
        Self {
            encryption_key: vec![0u8; 32],
            signing_key: vec![0u8; 32],
            auth_tokens: HashMap::new(),
            session_keys: HashMap::new(),
        }
    }

    pub fn initialize_keys(&mut self) -> Result<(), String> {
        // Generate random keys
        rand::thread_rng().fill_bytes(&mut self.encryption_key);
        rand::thread_rng().fill_bytes(&mut self.signing_key);
        
        info!("Cryptographic keys initialized");
        Ok(())
    }

    /// Simple XOR encryption for demonstration
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut encrypted = data.to_vec();
        
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= self.encryption_key[i % self.encryption_key.len()];
        }
        
        Ok(encrypted)
    }

    /// Simple XOR decryption for demonstration
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        let mut decrypted = encrypted_data.to_vec();
        
        for (i, byte) in decrypted.iter_mut().enumerate() {
            *byte ^= self.encryption_key[i % self.encryption_key.len()];
        }
        
        Ok(decrypted)
    }

    pub fn create_secure_message(&self, from: u32, to: u32, data: &[u8]) -> Result<Vec<u8>, String> {
        let encrypted = self.encrypt(data)?;
        let signature = self.sign_data(&encrypted)?;
        
        let mut message = Vec::new();
        message.extend_from_slice(&from.to_be_bytes());
        message.extend_from_slice(&to.to_be_bytes());
        message.extend_from_slice(&(signature.len() as u32).to_be_bytes());
        message.extend_from_slice(&signature);
        message.extend_from_slice(&encrypted);
        
        Ok(message)
    }

    pub fn verify_and_decrypt(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        if message.len() < 12 {
            return Err("Message too short".to_string());
        }
        
        let sig_len = u32::from_be_bytes([message[8], message[9], message[10], message[11]]) as usize;
        
        if message.len() < 12 + sig_len {
            return Err("Invalid message format".to_string());
        }
        
        let signature = &message[12..12 + sig_len];
        let encrypted_data = &message[12 + sig_len..];
        
        // Verify signature (simplified)
        let expected_sig = self.sign_data(encrypted_data)?;
        if signature != expected_sig {
            return Err("Signature verification failed".to_string());
        }
        
        self.decrypt(encrypted_data)
    }

    pub fn generate_auth_token(&mut self, node_id: u32, permissions: Vec<Permission>) -> Result<String, String> {
        let token_data = format!("{}:{:?}:{}", node_id, permissions, Utc::now().timestamp());
        let token_hash = format!("{:x}", Sha256::digest(token_data.as_bytes()));
        
        let expiry = Utc::now() + Duration::hours(24);
        self.auth_tokens.insert(node_id, (token_hash.clone(), expiry));
        
        Ok(token_hash)
    }

    pub fn verify_auth_token(&self, node_id: u32, token: &str, required_permission: Permission) -> Result<bool, String> {
        if let Some((stored_token, expiry)) = self.auth_tokens.get(&node_id) {
            if Utc::now() > *expiry {
                return Ok(false);
            }
            Ok(stored_token == token)
        } else {
            Ok(false)
        }
    }

    pub fn create_emergency_message(&self, node_id: u32, data: &[u8]) -> Result<Vec<u8>, String> {
        // Emergency messages use simplified encryption
        let mut message = Vec::new();
        message.extend_from_slice(b"EMERGENCY");
        message.extend_from_slice(&node_id.to_be_bytes());
        message.extend_from_slice(data);
        
        Ok(message)
    }

    pub fn verify_emergency_message(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        if message.len() < 13 || &message[0..9] != b"EMERGENCY" {
            return Err("Not an emergency message".to_string());
        }
        
        Ok(message[13..].to_vec())
    }

    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut hasher = Sha256::new();
        hasher.update(&self.signing_key);
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }
}