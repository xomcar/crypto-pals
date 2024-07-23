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
    let prepend_size: usize = rng.gen_range(1..100);
    let random_text: Vec<u8> = (0..prepend_size).map(|_| rng.gen()).collect();
    let real_block_len = 16;
    let fixed_key: Vec<u8> = (0..real_block_len).map(|_| rng.gen()).collect();
    let mut oracle = |data: &[u8]| {
        let mut text = random_text.clone();
        text.append(&mut data.to_vec());
        text.append(&mut plain_text.clone());
        encrypt_ecb(&text, &fixed_key)
    };

    // find block length
    let block_len = aes::find_block_len(&mut oracle, 40)?;
    assert_eq!(real_block_len, block_len);
    println!("Block length: {}", block_len);

    // find text length
    let text_len = aes::find_text_len(&mut oracle, block_len)?;
    assert_eq!(text_len, plain_text.len() + random_text.len());
    println!("Prepend + text length: {}", text_len);

    // check if using ecb
    let mut prepend_fill = 0;
    for i in 0..999 {
        let input = "a".repeat(2 * block_len + i);
        let output = oracle(input.as_bytes())?;
        if aes::count_duplicates(&output, block_len) > 0 {
            prepend_fill = block_len - i;
            break;
        }
    }
    if prepend_fill == block_len {
        prepend_fill = 0;
    }
    println!("Prepend fill: {}", prepend_fill);

    let empty = oracle(b"")?;
    let one = oracle(b"0")?;

    let mut last_pre_block = 0;
    for i in (0..empty.len()).step_by(block_len) {
        if empty[i..i + block_len] != one[i..i + block_len] {
            last_pre_block = i / block_len;
            break;
        }
    }

    let prepend_len = block_len * last_pre_block + prepend_fill;
    let end_of_block_padding = prepend_len % block_len;
    let mandatory_padding = block_len - end_of_block_padding;
    let target_size = text_len - prepend_len;
    assert_eq!(target_size, plain_text.len());
    assert_eq!(prepend_len, prepend_size);

    println!("Prepend size: {}", prepend_len);
    println!("Text size: {}", target_size);
    println!("Mandatory padding: {}", mandatory_padding);

    let mut decrypted_text = Vec::with_capacity(999);

    let start_index = prepend_len - prepend_fill;
    for i in start_index..text_len {
        let index;
        let padding_amount;
        // first row and the others require differnt handling
        if i < start_index + mandatory_padding {
            index = (block_len) * (i / (block_len) + 1) - 1;
            padding_amount = (mandatory_padding - 1) - (i % block_len);
        } else {
            index = (block_len) * ((i + prepend_fill) / (block_len) + 1) - 1;
            padding_amount = (block_len - 1) - ((i + prepend_fill) % block_len);
        }
        let mut attack_vector = "_".repeat(padding_amount).into_bytes();
        assert_eq!(index % block_len, block_len - 1);

        let actual_output = oracle(&attack_vector)?;

        attack_vector.append(&mut decrypted_text.clone());
        attack_vector.push(0);

        let atk_len = attack_vector.len();
        for b in 0..255 {
            attack_vector[atk_len - 1] = b;
            let attack_output = oracle(&attack_vector)?;

            let mut expected_output = actual_output[..padding_amount].to_vec();
            expected_output.append(&mut decrypted_text.clone());
            //println!("{:?}\n{:?}", &expected_output[..attack_index], &attack_output[..attack_index]);
            if attack_output[..index] == actual_output[..index] {
                decrypted_text.push(b);
                // println!("found: {} {}", b, b as char);
                break;
            }
        }
    }

    let stripped = aes::strip_pkcs7(&decrypted_text)?;

    println!("Decrypted text:\n{}", String::from_utf8(stripped)?);

    Ok(())
}
