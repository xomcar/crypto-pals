use crypto_bros::error::Result;
use crypto_bros::{aes::decrypt_cbc, io::cypher_text_from_base64_file};
pub fn main() -> Result<()> {
    let ct = cypher_text_from_base64_file("data/10.txt")?;
    let iv = [0u8; 16];
    let key = "YELLOW SUBMARINE".as_bytes();
    assert_eq!(key.len(), 16);
    assert_eq!(iv.len(), 16);
    let decrypted_data = decrypt_cbc(&ct, &key, &iv)?;
    let text = String::from_utf8(decrypted_data)?;
    println!("{}", text);
    Ok(())
}
