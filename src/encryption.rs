pub mod encryption {
    pub use crate::util::general;
    pub use crate::key_master::encrypt_key::{return_main_key, return_sub_key};
    use chrono::prelude::*;
    use aes::cipher::generic_array::GenericArray;
    use aes::Aes128;
    use aes_ctr::Aes256Ctr;
    use rand::rngs::OsRng;
    
    fn main() {
        let key = b"my-secret-key";
        let nonce = OsRng.next_u32().to_le_bytes();
    
        let plaintext = "Hello, world!";
        let ciphertext = encrypt(plaintext, key, nonce);
    
        let decrypted = decrypt(&ciphertext, key, nonce);
        assert_eq!(decrypted, plaintext);
    }
    
    fn encrypt(plaintext: &str, key: &[u8], nonce: [u8; 4]) -> Vec<u8> {
        let mut cipher = Aes256Ctr::new(&key.into(), &nonce.into());
        let mut ciphertext = plaintext.as_bytes().to_vec();
        cipher.apply_keystream(&mut ciphertext);
        ciphertext
    }
    
    fn decrypt(ciphertext: &[u8], key: &[u8], nonce: [u8; 4]) -> String {
        let mut cipher = Aes256Ctr::new(&key.into(), &nonce.into());
        let mut decrypted = ciphertext.to_vec();
        cipher.apply_keystream(&mut decrypted);
        String::from_utf8(decrypted).unwrap()
    }

    pub fn encrypt_file(path: &str, token: &str, nonce: [u8; 4]){
        let nonce = OsRng.next_u32().to_le_bytes();
        let subkey: &str = return_sub_key(token);
        let mainkey = return_main_key(subkey);

        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let line = line.unwrap();
            let encrypted_block = encrypt(line,mainkey.as_bytes(),nonce);
        }
    }

    pub fn decrypt_file(encrypted_data: &[u8], token: &[u8], nonce: &[u8]) {
        let nonce = OsRng.next_u32().to_le_bytes();
        let subkey: &str = return_sub_key(token);
        let mainkey = return_main_key(subkey);

        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines(){
            let line = line.unwrap();
            let encrypted_block = decrypt(line,mainkey.as_bytes(),nonce);
        }
    }
    pub fn return_timestamp(hash_id: &str) -> (String, String) {
        // Return Timestamp and the time being stamped
        let now = Local::now();
        let time = now.format("%H:%M:%S").to_string(); // convert to String
        let current_time = format!("{}&{}", hash_id, &time);
        let timestamp = general::hash_string(&current_time);
        (time, timestamp)
    }
}