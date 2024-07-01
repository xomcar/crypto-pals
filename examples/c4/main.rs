use std::fs;

use crypto_bros::{
    hex::{self, VecU8Ext},
    xor::xor,
    xor_cypher::get_score,
};

fn main() {
    let data = fs::read_to_string("data/4.txt").expect("File not found");
    let mut best_score = f32::MAX;
    let mut best_match = String::new();
    for input in data.lines() {
        let encrypted = hex::decode_hex(input).unwrap();
        for key in 0..u8::MAX {
            let secret = vec![key; encrypted.len()];
            let decrypted = xor(&secret, &encrypted);
            let score = get_score(&decrypted);
            if score < best_score {
                best_score = score;
                best_match = decrypted.to_ascii_string();
            }
        }
    }
    println!("{}", best_match);
}
