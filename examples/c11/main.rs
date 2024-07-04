use crypto_bros::aes::{encrypt_cbc, encrypt_ecb, has_duplicates};

pub fn main() {
    let pt = "a".repeat(64);
    let (result, kind) = enc_oracle(&pt.as_bytes());
    if has_duplicates(&result, 16) > 0 {
        assert_eq!(AESType::ECB, kind);
        println!("Successfully recognized ECB encryption")

    }
    else {
        assert_eq!(AESType::CBC, kind);
        println!("Successfully recognized CBC encryption")
    }
}

#[derive(PartialEq, Debug)]
pub enum AESType {
    ECB,
    CBC,
}

pub fn gen_random_data(len : usize) -> Vec<u8> {
    (0..len).map(|_| rand::random::<u8>()).collect()
}

pub fn enc_oracle(input : &[u8]) -> (Vec<u8>, AESType) {
    // generate padding length randomly
    let pad_len = (rand::random::<usize>() % 5) + 5;
    // generate random front padding
    let front_pad = gen_random_data(pad_len);
    // generate random back padding
    let back_pad = gen_random_data(pad_len);
    // pad input
    let mut padded_input = Vec::with_capacity(input.len() + 2*pad_len);
    padded_input.append(&mut input.to_vec());
    padded_input.append(&mut front_pad.to_vec());
    padded_input.append(&mut back_pad.to_vec());
    // choose encryption mode randomly
    let use_cbc = (rand::random::<u8>() % 2) == 0;
    if use_cbc {
        (cbc_enc_oracle(&padded_input), AESType::CBC)
    } else {
        (ecb_enc_oracle(&padded_input), AESType::ECB)
    }
}

pub fn cbc_enc_oracle(input : &[u8]) -> Vec<u8> {
    // generate random iv
    let iv = gen_random_data(16);
    let key = gen_random_data(16);
    encrypt_cbc(input, &key, &iv).unwrap()
}

pub fn ecb_enc_oracle(input : &[u8]) -> Vec<u8> {
    let key = gen_random_data(16);
    encrypt_ecb(input, &key).unwrap()
}