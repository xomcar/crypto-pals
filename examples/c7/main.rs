use crypto_bros::{aes, io};
pub fn main() {
    let ct = io::cypher_text_from_base64_file("data/7.txt");
    let secret_key = "YELLOW SUBMARINE";
    let unenc = aes::decrypt_ecb(&ct, secret_key.as_bytes());
    println!("{}", unenc);
}
