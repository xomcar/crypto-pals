use crate::base64;
use std::fs;

pub fn cypher_text_from_base64_file(path : &str) -> Vec<u8> {
    let base64_encoded_data = fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\n", "")))
        .expect("Error reading file");
    base64::decode(&base64_encoded_data).unwrap()
}