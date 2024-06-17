const BASE64_ENCODE_TABLE: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];
const FIRST_SIX: u8 = 0b1111_1100;
const LAST_SIX: u8 = 0b0011_1111;
const FIRST_FOUR: u8 = 0b1111_0000;
const LAST_FOUR: u8 = 0b0000_1111;
const FIRST_TWO: u8 = 0b1100_0000;
const LAST_TWO: u8 = 0b0000_0011;
const PADDING: u8 = 65;

pub fn to_base64(input: &[u8]) -> String {
    let mut result = String::new();
    let (mut first, mut second, mut third, mut fourth): (u8, u8, u8, u8);
    for chunk in input.chunks(3) {
        match chunk.len() {
            3 => {
                first = (chunk[0] & FIRST_SIX) >> 2;
                second = ((chunk[0] & LAST_TWO) << 4) | ((chunk[1] & FIRST_FOUR) >> 4);
                third = ((chunk[1] & LAST_FOUR) << 2) | ((chunk[2] & FIRST_TWO) >> 6);
                fourth = chunk[2] & LAST_SIX;
            }
            2 => {
                first = (chunk[0] & FIRST_SIX) >> 2;
                second = (chunk[0] & FIRST_TWO) << 6 | (chunk[1] & LAST_FOUR) >> 4;
                third = (chunk[1] & LAST_FOUR) << 2;
                fourth = PADDING;
            }
            1 => {
                first = (chunk[0] & FIRST_SIX) >> 2;
                second = (chunk[0] & FIRST_TWO) << 6;
                third = PADDING;
                fourth = PADDING;
            }
            _ => unreachable!("Can't go here"),
        }
        result.push(BASE64_ENCODE_TABLE[first as usize]);
        result.push(BASE64_ENCODE_TABLE[second as usize]);
        result.push(BASE64_ENCODE_TABLE[third as usize]);
        result.push(BASE64_ENCODE_TABLE[fourth as usize]);
    }
    return result;
}

#[test]
fn check_base64_algo() {
    use crate::{base64, hex};
    pub const EXAMPLE_INPUT : &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    pub const EXAMPLE_OUTPUT: &str =
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let input_string = hex::decode_hex(EXAMPLE_INPUT).unwrap();
    let result = base64::to_base64(input_string.as_slice());
    assert_eq!(result, EXAMPLE_OUTPUT)
}
