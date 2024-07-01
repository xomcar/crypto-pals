use crypto_bros::cypher;

const ENCRYPTED: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn main() {
    let guess = cypher::decrypt_single_byte_xor(ENCRYPTED);
    assert_eq!(guess, "Cooking MC's like a pound of bacon");
    println!("{}", guess)
}
