#!/bin/bash

set -e  # Hata durumunda scriptin durmasını sağlar

echo "Soğuk Cüzdan Kurulumuna Hoş Geldiniz!"
echo "Bu script, Rust'u ve gerekli tüm bağımlılıkları Linux üzerinde yükler ve projenizi hazırlar."

# Gerekli sistem paketlerini kontrol et ve yükle
echo "1. Gerekli paketler yükleniyor..."
sudo apt update
sudo apt install -y curl build-essential libssl-dev pkg-config

# Rust kurulu mu kontrol et
if ! command -v rustc &> /dev/null; then
    echo "2. Rust kurulumu başlatılıyor..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust zaten yüklü."
fi

# Proje klasörü kontrol et
if [ ! -d "./soguk-cuzdan" ]; then
    echo "3. Soğuk Cüzdan proje dosyası oluşturuluyor..."
    mkdir soguk-cuzdan
    cd soguk-cuzdan
    cargo new soguk-cuzdan
    cd soguk-cuzdan
else
    echo "Proje klasörü zaten mevcut. Devam ediliyor..."
    cd soguk-cuzdan
fi

# Cargo.toml için bağımlılıkları ekle
echo "4. Cargo.toml bağımlılıkları ekleniyor..."
cat <<EOT > Cargo.toml
[package]
name = "soguk-cuzdan"
version = "0.1.0"
edition = "2021"

[dependencies]
aes-gcm = "0.10"
ed25519-dalek = "1.0"
pbkdf2 = "0.10"
sha2 = "0.10"
rand = "0.8"
zeroize = "1.5"
hex = "0.4"
EOT

# Proje için temel kodları oluştur
echo "5. Temel Rust dosyaları oluşturuluyor..."
mkdir -p src
cat <<EOT > src/main.rs
mod crypto;
mod user;

fn main() {
    println!("Soğuk Cüzdan Uygulamasına Hoş Geldiniz!");
}
EOT

cat <<EOT > src/crypto.rs
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-256 GCM
use aes_gcm::aead::{Aead, NewAead};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use pbkdf2::{pbkdf2_hmac, password_hash::SaltString};
use rand::rngs::OsRng;
use sha2::Sha256;
use zeroize::Zeroize;

const AES_KEY_SIZE: usize = 32;
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

        keypair.secret.as_bytes().zeroize();

        signature
    }
}
EOT

# Projeyi derle ve çalıştır
echo "6. Proje derleniyor..."
cargo build --release

echo "Kurulum Tamamlandı! Soğuk Cüzdan uygulamanız hazır."
echo "Çalıştırmak için: ./target/release/soguk-cuzdan"
