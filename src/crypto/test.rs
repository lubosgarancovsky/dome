use super::*;
const MASTER_PASSWORD: &str = "mAste&rPass123word";
const DOMAIN_PASSWORD: &str = "encRyp73dPa55w0rd";

#[test]
fn test_encrypt_decrypt() {
    let salt = generate_salt();
    let key = derive_key(MASTER_PASSWORD, &salt);
    let (hash, nonce) = encrypt(&key, DOMAIN_PASSWORD);

    let second_key = derive_key(MASTER_PASSWORD, &salt);
    let pwd = decrypt(&second_key, &nonce, &hash);

    assert_eq!(DOMAIN_PASSWORD, pwd);
}
