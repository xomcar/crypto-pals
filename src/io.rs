use crate::base64;
use crate::error::Result;
use std::fs;

pub fn cypher_text_from_base64_file(path: &str) -> Result<Vec<u8>> {
    let mut base64_encoded_data = fs::read_to_string(path)?;
    base64_encoded_data.retain(|c| !c.is_ascii_whitespace());
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
