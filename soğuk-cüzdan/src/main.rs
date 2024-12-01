mod crypto;
mod user;

use user::UserManager;

fn main() {
    // Kullanıcı menüsü.
    println!("1. Kayıt Ol");
    println!("2. Giriş Yap");

    let choice = "1"; // Bu kullanıcı girişinden alınabilir.

    if choice == "1" {
        let password = "user_password"; // Kullanıcıdan alınacak.
        let use_custom_keys = false;

        let (pin, seed) =
            UserManager::register_user(password, use_custom_keys, None, None);
        println!("Kayıt tamamlandı! PIN: {}, Seed: {}", pin, seed);
    } else if choice == "2" {
        let password = "user_password"; // Kullanıcıdan alınacak.
        let pin = "123456"; // Kullanıcıdan alınacak.

        let success = UserManager::login_user(password, pin);
        if success {
            println!("Giriş başarılı!");
        } else {
            println!("Giriş başarısız!");
        }
    }
}
