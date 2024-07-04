use std::fs;

use crypto_bros::{hex, xor::fixed_xor, xor::get_english_lang_score};

const SOLUTION: &str = "Now that the party is jumping";

fn main() {
    let data = fs::read_to_string("data/4.txt").expect("File not found");
    let mut best_score = f32::MAX;
    let mut best_match: Vec<u8> = vec![];
    for input in data.lines() {
        let encrypted = hex::decode(input).unwrap();
        for key in 0..u8::MAX {
            let secret = vec![key; encrypted.len()];
            let decrypted = fixed_xor(&secret, &encrypted);
            let score = get_english_lang_score(&decrypted);
            if score < best_score {
                best_score = score;
                best_match = decrypted;
            }
        }
    }
    let result = String::from_utf8(best_match).unwrap();
    assert_eq!(result.trim(), SOLUTION);
    println!("{}", result.trim());
}
