use std::{fmt::Write, num::ParseIntError};

pub fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    return (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
}

pub fn encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    return s;
}

pub trait VecU8Ext {
    fn to_ascii_string(&self) -> String;
}

impl VecU8Ext for Vec<u8> {
    fn to_ascii_string(&self) -> String {
        self.iter()
            .map(|&b| {
                if b <= 127 {
                    b as char
                } else {
                    '?' // Replace non-ASCII characters with '?'
                }
            })
            .collect::<String>()
    }
}
