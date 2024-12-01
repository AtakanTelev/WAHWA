use crate::crypto::CryptoManager;
use ed25519_dalek::{Keypair, SecretKey};
use rand::rngs::OsRng;
use std::fs::{self, File};
use std::io::{Read, Write};
use zeroize::Zeroize; // RAM sıfırlama

pub struct UserManager;

impl UserManager {
    pub fn register_user(
        password: &str,
        use_custom_keys: bool,
        custom_public: Option<String>,
        custom_private: Option<String>,
    ) -> (String, String) {
        let salt = "unique_salt"; // Salt sabit tutulabilir, daha iyi bir yaklaşımla dinamik yapabilirsiniz.
        let encryption_key = CryptoManager::pbkdf2_derive_key(password, salt);

        let keypair = if use_custom_keys {
            let public_key = custom_public.expect("Public key is required");
            let private_key = custom_private.expect("Private key is required");
            let secret_key_bytes = hex::decode(private_key).expect("Invalid private key");
            let secret_key = SecretKey::from_bytes(&secret_key_bytes)
                .expect("Failed to create secret key");
            Keypair {
                public: ed25519_dalek::PublicKey::from_bytes(
                    &hex::decode(public_key).expect("Invalid public key"),
                )
                .expect("Failed to create public key"),
                secret: secret_key,
            }
        } else {
            CryptoManager::generate_keypair()
        };

        // Anahtarları şifrele ve dosyaya kaydet
        let encrypted_private_key = CryptoManager::encrypt_data(
            keypair.secret.as_bytes(),
            &encryption_key,
            b"unique_nonce_1",
        );

        let encrypted_public_key =
            CryptoManager::encrypt_data(keypair.public.as_bytes(), &encryption_key, b"unique_nonce_2");

        fs::write("private_key.dat", encrypted_private_key).expect("Failed to write private key");
        fs::write("public_key.dat", encrypted_public_key).expect("Failed to write public key");

        // PIN ve seed oluştur
        let pin = format!("{:06}", rand::random::<u32>() % 1_000_000);
        let seed = "example_seed_phrase_12_or_20_words".to_string(); // Gerçekte dinamik bir seed oluşturulmalı.

        // Hassas verileri sıfırla
        keypair.secret.as_bytes().zeroize();
        encryption_key.zeroize();

        (pin, seed)
    }

    pub fn login_user(password: &str, pin: &str) -> bool {
        let salt = "unique_salt";
        let encryption_key = CryptoManager::pbkdf2_derive_key(password, salt);

        let encrypted_private_key =
            fs::read("private_key.dat").expect("Failed to read private key file");
        let encrypted_public_key =
            fs::read("public_key.dat").expect("Failed to read public key file");

        // Anahtarları çöz
        let private_key = CryptoManager::decrypt_data(&encrypted_private_key, &encryption_key, b"unique_nonce_1");
        let public_key = CryptoManager::decrypt_data(&encrypted_public_key, &encryption_key, b"unique_nonce_2");

        // RAM'den sıfırla
        encryption_key.zeroize();
        private_key.zeroize();
        public_key.zeroize();

        // PIN kontrolü
        pin == "correct_pin" // Bunu dinamik yapabilirsiniz.
    }
}
