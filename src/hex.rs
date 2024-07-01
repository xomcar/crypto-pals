use std::fmt::Write;

pub fn decode(hex: &str) -> Result<Vec<u8>, &'static str> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even length");
    }

    let mut output = Vec::new();

    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex string")?;
        output.push(byte);
    }

    Ok(output)
}

pub fn encode(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for byte in data {
        write!(&mut s, "{:02x}", byte).unwrap();
    }
    s
}

#[test]
pub fn test_hex() {
    let encoded = encode(&[0xDE, 0xAD]);
    let decoded = decode("DEAD").unwrap();
    assert_eq!(encoded, "DEAD");
    assert_eq!(decoded, [0xDE, 0xAD]);
}
