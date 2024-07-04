use crypto_bros::{aes::has_duplicates, hex, io::cypher_texts_from_base64_file};

pub fn main() {
    let cts = cypher_texts_from_base64_file("data/8.txt");
    let mut encoded_index = usize::MAX;
    let mut max_same = 0;
    for (ct_index, ct) in cts.iter().enumerate() {
        let dups = has_duplicates(ct, 16);
        if dups > max_same {
            max_same = dups;
            encoded_index = ct_index;
        }
    }
    let encoded_data = hex::encode(&cts[encoded_index]);
    println!("AES ECB encoded at row {}: \n{}", encoded_index, encoded_data)
}