use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-256 GCM
use aes_gcm::aead::{Aead, NewAead};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use pbkdf2::{pbkdf2_hmac, password_hash::SaltString};
use rand::rngs::OsRng;
use sha2::Sha256;
use zeroize::Zeroize; // RAM sıfırlama için gerekli kütüphane
use std::fs::{self, File};
use std::io::{Read, Write};

const AES_KEY_SIZE: usize = 32; // 256-bit key
const PBKDF2_ITERATIONS: u32 = 100_000;

pub struct CryptoManager;

impl CryptoManager {
    pub fn generate_keypair() -> Keypair {
        Keypair::generate(&mut OsRng)
    }

    pub fn encrypt_data(data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        cipher.encrypt(Nonce::from_slice(nonce), data).expect("Encryption failed")
    }

    pub fn decrypt_data(encrypted_data: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        cipher.decrypt(Nonce::from_slice(nonce), encrypted_data).expect("Decryption failed")
    }

    pub fn pbkdf2_derive_key(password: &str, salt: &str) -> Vec<u8> {
        let mut key = vec![0u8; AES_KEY_SIZE];
        pbkdf2_hmac(
            password.as_bytes(),
            salt.as_bytes(),
            PBKDF2_ITERATIONS,
            Sha256::new(),
            &mut key,
        );

        // Hassas belleği sıfırlama
        password.zeroize();
        salt.zeroize();

        key
    }

    pub fn sign_data(data: &[u8], secret_key: &SecretKey) -> Signature {
        let keypair = Keypair {
            secret: secret_key.clone(),
            public: PublicKey::from(secret_key),
        };
        let signature = keypair.sign(data);

        // Hassas verileri RAM'den sıfırla
        keypair.secret.as_bytes().zeroize();

        signature
    }
}
