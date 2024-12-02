# Cold Wallet Application

Note: Application is actually doesn't work rigth now.

This project aims to provide a secure cold wallet solution operating without internet or USB connectivity. It is designed to run on a Linux system within VirtualBox. The application supports the following features and standards.

## Features

### Supported File Types
- **Wallet Files**: `.DAT`, `.JSON`, `.KEY`, `.PEM`
- **Transaction Files**: `.TXN`, `.RAW`, `.PSBT`
- **Backup Files**: `.ZIP`, `.7Z`, `.PDF`

### Encryption and Security
- **Salt Usage**: Enhances password and key security.
- **Ed25519 Encryption**: Used for signing operations.
- **AES-256 or SHA-256**: Used for data encryption.
- **PBKDF2**: Handles key derivation and management.
- Memory is cleared after each operation to ensure data safety.

### Registration and Login System
1. **Registration Options**
   - Users can either provide their public and private keys or let the program generate them automatically.
2. **PIN and Seed**
   - A 6-digit PIN is set after registration.
   - Users receive a 12 or 20-word seed for recovery.
3. **Login**
   - Users log in using their PIN.
   - After 5 failed attempts, wallet files are deleted.
4. **Recovery**
   - Public and private keys can be recovered using the seed.

### Signature Operations
- Signing is performed for transaction data.
- The signature output is displayed to the user.

---

### Project Structure
- `kayit.rs`: Handles registration processes.
- `giris.rs`: Manages login processes.
- `imza.rs`: Contains the signing operations.
- `main.rs`: Combines all modules and manages application flow.

---

### Usage
1. Create a Linux machine in VirtualBox.
2. Ensure Rust is installed on the system.
3. Run the application using the command `cargo run`.

### Problems

- what happend when electric is gone
- SSD's own frailtys

You can add a comment if you can in github i don't know about github or add on read.me note.

# Soğuk Cüzdan Uygulaması

Not: Bu App daha çalışmamaktadır.

Bu proje, internet veya USB bağlantısı olmadan güvenli bir soğuk cüzdan çözümü sunmayı amaçlamaktadır. VirtualBox üzerinde çalışan bir Linux sistemi için geliştirilmiştir. Uygulama, aşağıdaki özellikleri ve standartları destekler.

## Özellikler

### Desteklenen Dosya Türleri
- **Cüzdan Dosyaları**: `.DAT`, `.JSON`, `.KEY`, `.PEM`
- **İşlem Dosyaları**: `.TXN`, `.RAW`, `.PSBT`
- **Yedekleme Dosyaları**: `.ZIP`, `.7Z`, `.PDF`

### Şifreleme ve Güvenlik
- **Salt Kullanımı**: Parola ve anahtar güvenliği için kullanılır.
- **Ed25519 Şifreleme**: İmza işlemleri için kullanılır.
- **AES-256 veya SHA-256**: Veri şifreleme için kullanılır.
- **PBKDF2**: Anahtar türetme ve yönetiminde kullanılır.
- Bellek güvenliği sağlamak için işlemler sonrası RAM sıfırlanır.

### Kayıt ve Giriş Sistemi
1. **Kayıt Seçenekleri**
   - Kullanıcı, public ve private keylerini kendisi girebilir veya program otomatik oluşturabilir.
2. **Pin ve Seed**
   - Kayıt işlemi sonrası 6 basamaklı bir PIN belirlenir.
   - Kullanıcıya 12 veya 20 kelimeden oluşan bir seed verilir.
3. **Giriş**
   - PIN ile giriş yapılır.
   - PIN 5 kez yanlış girilirse cüzdan dosyaları silinir.
4. **Kurtarma**
   - Seed kullanılarak public ve private keyler kurtarılabilir.

### İmza İşlemi
- Para transferi için işlem verisi imzalanır.
- İmza çıktısı kullanıcıya ekranda gösterilir.

---

### Proje Yapısı
- `kayit.rs`: Kayıt işlemlerini gerçekleştirir.
- `giris.rs`: Giriş işlemlerini yönetir.
- `imza.rs`: İmza oluşturma işlemlerini içerir.
- `main.rs`: Tüm modülleri birleştirir ve uygulama akışını yönetir.

---

### Kullanım
1. VirtualBox üzerinde bir Linux makinesi oluşturun.
2. Uygulamayı çalıştırmak için Rust kurulumunu tamamlayın.
3. `cargo run` komutunu kullanarak uygulamayı başlatın.

### sorunlar 

- Elektirik kesintisinde ne yapacaksın 
- SSD'nin kendi açıkları

Fikrin kendi sorunlarını read.me ekliyebilir yada github yorum yazma varsa oraya yazabilirsiniz.
