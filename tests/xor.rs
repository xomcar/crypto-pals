use crypto_bros::{hex, xor};

pub const EXAMPLE_INPUT_1: &str = "1c0111001f010100061a024b53535009181c";
pub const EXAMPLE_INPUT_2: &str = "686974207468652062756c6c277320657965";
pub const EXAMPLE_OUTPUT: &str = "746865206b696420646f6e277420706c6179";

#[test]
fn check_xor_algo() {
    let input1 = hex::decode_hex(EXAMPLE_INPUT_1).unwrap();
    let input2 = hex::decode_hex(EXAMPLE_INPUT_2).unwrap();
    let expected = hex::decode_hex(EXAMPLE_OUTPUT).unwrap();
    let output = xor::xor(&input1, &input2);
    assert_eq!(expected, output);
}
