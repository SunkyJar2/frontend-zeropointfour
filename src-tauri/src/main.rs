use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{SaltString, PasswordHash};
use rand_core::OsRng;
use base64::{engine::general_purpose, Engine as _};

#[tauri::command]
fn hash_password(password: String) -> String {
    println!("[hash_password] Received password: {}", password);

    let salt = SaltString::generate(&mut OsRng);
    println!("[hash_password] Generated salt: {}", salt);

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    println!("[hash_password] Generated hash: {}", hash);

    hash
}

#[tauri::command]
fn verify_password(password: String, hash: String) -> bool {
    println!("[verify_password] Verifying password: {}", password);
    println!("[verify_password] Against hash: {}", hash);

    let parsed_hash = PasswordHash::new(&hash).unwrap();

    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    println!("[verify_password] Verification result: {}", result);

    result
}

#[tauri::command]
fn derive_key(password: String, salt: String) -> String {
    let argon2 = Argon2::default();
    let mut output_key = [0u8; 32];

    argon2
        .hash_password_into(
            password.as_bytes(),
            salt.as_bytes(),       
            &mut output_key,
        )
        .unwrap();

    base64::engine::general_purpose::STANDARD.encode(output_key)
}

fn main() {
    println!("[main] Starting Tauri app...");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hash_password,
            verify_password,
            derive_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("[main] Tauri app shutdown.");
}
