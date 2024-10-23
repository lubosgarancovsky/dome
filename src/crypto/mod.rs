use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::Argon2;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use rand::Rng;

pub const KEY_SIZE: usize = 32;
pub const SALT_SIZE: usize = 16;

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; SALT_SIZE];
    rand::thread_rng().fill(&mut salt[..]);
    salt
}

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_SIZE] {
    let argon2 = Argon2::default();
    let mut key: [u8; KEY_SIZE] = [0u8; KEY_SIZE];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("Key derivation failed");
    key
}

pub fn encrypt(key: &[u8; KEY_SIZE], password: &str) -> (String, Vec<u8>) {
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);

    let binding = rand::random::<[u8; 12]>();
    let nonce = Nonce::from_slice(&binding);
    let cipher_text = cipher
        .encrypt(nonce, password.as_bytes())
        .expect("Encryption failed");

    (STANDARD_NO_PAD.encode(cipher_text), nonce.to_vec())
}

pub fn decrypt(key: &[u8], nonce: &[u8], encoded_pwd: &str) -> String {
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);

    let decoded_cipher_text = STANDARD_NO_PAD
        .decode(encoded_pwd)
        .expect("Base64 decoding failed.");
    let plain_text = cipher
        .decrypt(Nonce::from_slice(nonce), decoded_cipher_text.as_ref())
        .expect("Decryption failed");

    String::from_utf8(plain_text).expect("Invalid UTF-8")
}

#[cfg(test)]
mod test;
