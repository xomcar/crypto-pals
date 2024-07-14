//Detect single-character XOR
use crypto_bros::{
    bench,
    error::Result,
    hex,
    xor::{appy_fixed, get_english_lang_score},
};
use std::fs;

fn solve() -> Result<()> {
    let expected_str = "Now that the party is jumping\n";
    let data = fs::read_to_string("data/4.txt")?;
    let mut best_score = f32::MAX;
    let mut best_match: Vec<u8> = vec![];
    let mut best_key: u8 = 0;
    for input in data.lines() {
        let encrypted = hex::decode(input)?;
        for key in 0..u8::MAX {
            let secret = vec![key; encrypted.len()];
            let decrypted = appy_fixed(&secret, &encrypted);
            let score = get_english_lang_score(&decrypted);
            if score < best_score {
                best_score = score;
                best_match = decrypted;
                best_key = key;
            }
        }
    }
    let result_str = String::from_utf8(best_match)?;
    assert_eq!(result_str, expected_str);
    println!(
        "found key:\n\t{}\nwith score:\n\t{}\ndecrypted text:\n\t{}",
        best_key, best_score, result_str
    );
    Ok(())
}

pub fn main() -> Result<()> {
    bench::time(&solve)
}
