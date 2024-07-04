use std::fs;

use crypto_bros::{base64, xor};

pub fn main() {
    let mut base64_encoded_data = fs::read_to_string("data/6.txt").expect("File not found");
    base64_encoded_data.retain(|c| !c.is_ascii_whitespace());
    let xor_text_data = base64::decode(&base64_encoded_data).unwrap();
    let key_sizes = xor::guess_keysize(&xor_text_data, 1);
    for (_, ksz) in key_sizes {
        let mut key: Vec<u8> = Vec::with_capacity(ksz);
        for i in 0..ksz {
            let picked: Vec<u8> = xor_text_data
                .clone()
                .into_iter()
                .skip(i)
                .step_by(ksz)
                .collect();
            let (_, key_guess) = xor::crack_single_byte_xor(&picked);
            key.push(key_guess);
        }
        let unenc = xor::repeating_xor(&xor_text_data, &key);
        println!(
            "================ Guessed key size: {} =====================",
            ksz
        );
        let key_str = String::from_utf8(key).unwrap();
        println!("Guessed key: {}", key_str);
        println!("===========================================================");
        println!("{}", String::from_utf8(unenc).unwrap());

        assert_eq!(key_str, "Terminator X: Bring the noise")
    }
}
