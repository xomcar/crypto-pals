//Single-byte XOR cipher
use crypto_bros::{bench, error::Result, hex, xor};

pub fn solve() -> Result<()> {
    let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let enc_data = hex::decode(hex_input)?;
    let (dec_data, key) = xor::crack_single_byte(&enc_data);

    let enc_str = String::from_utf8(enc_data)?;
    let dec_str = String::from_utf8(dec_data)?;
    assert_eq!(dec_str, "Cooking MC's like a pound of bacon");
    println!(
        "encrypted input:\n\nt{}\ncracked key:\n\t{}\ndecrypted message:\n\t{}",
        enc_str, key, dec_str,
    );
    Ok(())
}

pub fn main() -> Result<()> {
    bench::time(&solve)
}
