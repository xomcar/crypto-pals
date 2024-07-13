use crypto_bros::{hex, xor};

const ENCRYPTED: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn main() {
    let input_data = hex::decode(ENCRYPTED).unwrap();
    let guess = xor::crack_single_byte_xor(&input_data).0;
    let result = String::from_utf8(guess).unwrap();
    assert_eq!(result, "Cooking MC's like a pound of bacon");
    println!("{}", result)
}
