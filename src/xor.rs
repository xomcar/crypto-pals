pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.len() != b.len() {
        panic!("cannot xor different lenght buffer!")
    } else {
        return a
            .into_iter()
            .zip(b.into_iter())
            .map(|(x, y)| x ^ y)
            .collect();
    }
}

#[test]
pub fn check_xor_algo() {
    use crate::{hex, xor};
    const EXAMPLE_INPUT_1: &str = "1c0111001f010100061a024b53535009181c";
    const EXAMPLE_INPUT_2: &str = "686974207468652062756c6c277320657965";
    const EXAMPLE_OUTPUT: &str = "746865206b696420646f6e277420706c6179";
    let input1 = hex::decode_hex(EXAMPLE_INPUT_1).unwrap();
    let input2 = hex::decode_hex(EXAMPLE_INPUT_2).unwrap();
    let expected = hex::decode_hex(EXAMPLE_OUTPUT).unwrap();
    let output = xor::xor(&input1, &input2);
    assert_eq!(expected, output);
}
