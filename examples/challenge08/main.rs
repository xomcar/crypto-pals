//Detect AES in ECB mode
use crypto_bros::error::Result;
use crypto_bros::{aes::count_duplicates, hex, io::cypher_texts_from_base64_file};
pub fn main() -> Result<()> {
    let cypher_texts = cypher_texts_from_base64_file("data/8.txt")?;
    let expected_row = 132;
    let mut encoded_row = usize::MAX;
    let mut max_same = 0;
    for (row, cypher_text) in cypher_texts.iter().enumerate() {
        let dups = count_duplicates(cypher_text, 16);
        if dups > max_same {
            max_same = dups;
            encoded_row = row;
        }
    }
    let encoded_data = hex::encode(&cypher_texts[encoded_row]);
    assert_eq!(expected_row, encoded_row);
    println!("AES ECB encoded at row {}: \n{}", encoded_row, encoded_data);
    Ok(())
}
