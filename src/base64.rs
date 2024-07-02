const BASE64_ENCODE_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const PADDING: char = '=';

pub fn encode(data: &[u8]) -> String {
    let mut encoded = String::new();
    let mut i: usize = 0;

    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };

        let index0 = b0 >> 2;
        let index1 = ((b0 & 0b00000011) << 4) | (b1 >> 4);
        let index2 = ((b1 & 0b00001111) << 2) | (b2 >> 6);
        let index3 = b2 & 0b00111111;

        encoded.push(BASE64_ENCODE_TABLE[index0 as usize]);
        encoded.push(BASE64_ENCODE_TABLE[index1 as usize]);

        if i + 1 < data.len() {
            encoded.push(BASE64_ENCODE_TABLE[index2 as usize]);
        } else {
            encoded.push(PADDING);
        }

        if i + 2 < data.len() {
            encoded.push(BASE64_ENCODE_TABLE[index3 as usize]);
        } else {
            encoded.push(PADDING);
        }

        i += 3;
    }

    encoded
}

pub fn decode(encoded: &str) -> Result<Vec<u8>, &'static str> {
    let mut decoded = vec![];
    let mut buffer = [0u8; 4];
    let mut buffer_len = 0;

    for c in encoded.chars() {
        if c == PADDING {
            break;
        }
        if let Ok(number) = match c {
            'A'..='Z' => Ok(c as u8 - 'A' as u8),
            'a'..='z' => Ok(c as u8 - 'a' as u8 + 26),
            '0'..='9' => Ok(c as u8 - '0' as u8 + 52),
            '+' => Ok(62),
            '/' => Ok(63),
            '=' => Ok(64),  // Padding character
            _ => Err(format!("Invalid character in Base64 string {}", c)),
        }{
            buffer[buffer_len] = number;
            buffer_len += 1;

            if buffer_len == 4 {
                let first = (buffer[0] << 2) | (buffer[1] >> 4);
                let second = ((buffer[1] & 0x0F) << 4 ) | (buffer[2] >> 2);
                let third = ((buffer[2] & 0x03) << 6) | buffer[3];

                decoded.push(first);
                if buffer[2] != 64 {
                    decoded.push(second);
                }
                if buffer[3] != 64 {
                    decoded.push(third);
                }
                buffer_len = 0;
            }
        } else {
            return Err("Invalid base64 string");
        }
    }

    if buffer_len == 3 {
        let first = (buffer[0] << 2) | (buffer[1] >> 4);
        let second = ((buffer[1] & 0x0F) << 4) | (buffer[2] >> 2);
        decoded.push(first);
        if buffer[2] != PADDING as u8 {
            decoded.push(second);
        }
    } else if buffer_len == 2 {
        let first = (buffer[0] << 2) | (buffer[1] >> 4);
        decoded.push(first);
    }
    
    Ok(decoded)
}


#[test]
fn check_base64_algo() {
    use crate::{base64, hex};
    pub const EXAMPLE_INPUT : &str = 
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    pub const EXAMPLE_OUTPUT: &str =
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let hex_decoded = hex::decode(EXAMPLE_INPUT).unwrap();
    let encoded_result = base64::encode(&hex_decoded);
    let decoded_result = base64::decode(EXAMPLE_OUTPUT).unwrap();
    let decoded_result_hex = hex::encode(&decoded_result);
    assert_eq!(encoded_result, EXAMPLE_OUTPUT);
    assert_eq!(decoded_result_hex, EXAMPLE_INPUT);
}
