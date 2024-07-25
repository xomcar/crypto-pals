// Break repeating-key XOR
use crypto_bros::{bench, error::Result, io, xor};

pub fn solve() -> Result<()> {
    let expected_key = "Terminator X: Bring the noise";
    let enc_data = io::cypher_text_from_base64_file("data/6.txt")?;
    let (_, key_size) = xor::guess_keysize(&enc_data, 1)?[0];
    let mut key: Vec<u8> = Vec::with_capacity(key_size);
    for i in 0..key_size {
        let picked: Vec<u8> = enc_data
            .clone()
            .into_iter()
            .skip(i)
            .step_by(key_size)
            .collect();
        let (_, key_guess) = xor::crack_single_byte(&picked)?;
        key.push(key_guess);
    }
    let dec_data = xor::apply_repeating(&enc_data, &key)?;
    let key_str = String::from_utf8(key)?;
    let dec_str = String::from_utf8(dec_data)?;
    assert_eq!(key_str, expected_key);

    println!(
        "cracked cipher with key:\n\t{}\ntext:\n{}",
        key_str, dec_str
    );
    Ok(())
}

pub fn main() -> Result<()> {
    let (res, duration) = bench!(solve());
    println!("took: {}ms", duration.as_millis());
    res
}
