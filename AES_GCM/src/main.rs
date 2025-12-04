use aes_gcm::{
    Aes256Gcm, KeyInit, Key, Nonce,
    aead::{Aead}
};
use rand::RngCore;
use serde::{Serialize, Deserialize};
use sled::Db;

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    username: String,
    level: u32,
    coins: u64,
}

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn encrypt(key: &[u8; 32], data: &str) -> Vec<u8> {

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut encrypted = cipher.encrypt(nonce, data.as_bytes()).unwrap();
    encrypted.extend_from_slice(&nonce_bytes);
    encrypted
}

fn decrypt(key: &[u8; 32], encrypted: &[u8]) -> String {

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let split = encrypted.len() - 12;
    let (ciphertext, nonce_bytes) = encrypted.split_at(split);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = cipher.decrypt(nonce, ciphertext).unwrap();
    String::from_utf8(decrypted).unwrap()
}

fn save(db: &Db, key: &str, value: &[u8]) {
    db.insert(key, value).unwrap();
    db.flush().unwrap();
}

fn main() {
    let db = sled::open("encrypted_db").unwrap();

    let key = generate_key();

    let user = UserData {
        username: "ShadowTitan".to_string(),
        level: 99,
        coins: 123_456,
    };

    let json = serde_json::to_string(&user).unwrap();
    let encrypted = encrypt(&key, &json);

    save(&db, "user_data", &encrypted);
    println!("Data saved securely!");

    let loaded_encrypted = db.get("user_data").unwrap().unwrap();
    let decrypted_json = decrypt(&key, &loaded_encrypted);

    let loaded_user: UserData = serde_json::from_str(&decrypted_json).unwrap();
    println!("Loaded user: {:?}", loaded_user);
}
