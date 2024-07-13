use crate::base64;
use crate::error::Result;
use std::fs;

pub fn cypher_text_from_base64_file(path: &str) -> Result<Vec<u8>> {
    let base64_encoded_data = fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\r", "").replace("\n", "")))
        .expect("Error reading file");
    base64::decode(&base64_encoded_data)
}

pub fn cypher_texts_from_base64_file(path: &str) -> Result<Vec<Vec<u8>>> {
    let output = fs::read_to_string(path)?
        .lines()
        .into_iter()
        .map(|l| base64::decode(&l).unwrap())
        .collect();
    Ok(output)
}
