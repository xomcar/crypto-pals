use crypto_bros::aes::{self, encrypt_ecb};
use crypto_bros::base64;
use crypto_bros::error::Result;
use rand::Rng;

pub fn main() -> Result<()> {
    solve()
}

fn solve() -> Result<()> {
    let text : &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let plain_text = base64::decode(text)?;
    let mut rng = rand::thread_rng();

    let real_block_len = 16;
    let fixed_key: Vec<u8> = (0..real_block_len).map(|_| rng.gen()).collect();
    let mut oracle_ecb_fixed = |data: &[u8]| {
        let mut padded = data.to_vec();
        padded.append(&mut plain_text.clone());
        encrypt_ecb(&padded, &fixed_key)
    };
    let block_len = aes::find_block_len(&mut oracle_ecb_fixed, 40)?;

    // find block length
    assert_eq!(real_block_len, block_len);
    println!("Block length: {}", block_len);

    // find text length
    let text_len = aes::find_text_len(&mut oracle_ecb_fixed, block_len)?;
    assert_eq!(text_len, plain_text.len());
    println!("Text length: {}", text_len);

    // check if using ecb
    let input = "a".repeat(block_len * 2);
    let output = oracle_ecb_fixed(input.as_bytes())?;
    assert_ne!(aes::count_duplicates(&output, block_len), 0);
    println!("Using ECB encryption");

    let mut decrypted_text = Vec::with_capacity(text_len);
    for i in 0..text_len {
        let padding_amount = (block_len - 1) - (i % block_len);
        let mut attack_vector = "_".repeat(padding_amount).into_bytes();
        let attack_index = (block_len) * (i / (block_len) + 1) - 1;

        let actual_output = oracle_ecb_fixed(&attack_vector)?;

        attack_vector.append(&mut decrypted_text.clone());
        attack_vector.push(0);
        // println!("padding: {} atk index: {} attack_vector len: {}\n{:?}",
        //     padding_amount, attack_index, attack_vector.len(), attack_vector);

        for b in 0..255 {
            attack_vector[attack_index] = b;
            let attack_output = oracle_ecb_fixed(&attack_vector)?;

            let mut expected_output = actual_output[..padding_amount].to_vec();
            expected_output.append(&mut decrypted_text.clone());
            //println!("{:?}\n{:?}", &expected_output[..attack_index], &attack_output[..attack_index]);
            if attack_output[..attack_index] == actual_output[..attack_index] {
                decrypted_text.push(b);
                //println!("found: {} {}", b, b as char);
                break;
            }
        }
    }

    let stripped = aes::strip_pkcs7(&decrypted_text)?;
    println!("Decrypted text:\n{}", String::from_utf8(stripped)?);

    Ok(())
}
