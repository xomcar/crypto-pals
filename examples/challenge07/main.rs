// AES in ECB mode
use crypto_bros::error::Result;
use crypto_bros::{aes, io};

pub fn main() -> Result<()> {
    let cypher_text = io::cypher_text_from_base64_file("data/7.txt")?;
    let secret_key = "YELLOW SUBMARINE";
    let dec_data = aes::decrypt_ecb(&cypher_text, secret_key.as_bytes())?;
    let dec_str = String::from_utf8(dec_data)?;
    println!("key:\n\t{}\ndecrypted text:\n{}", secret_key, dec_str);
    Ok(())
}
