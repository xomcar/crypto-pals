use crate::error::Result;
use crate::xor::appy_fixed;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, NewBlockCipher};
use aes::{Aes128, BlockEncrypt};

pub fn encrypt_cbc(pt: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let chunk_size = key.len();
    if iv.len() != chunk_size {
        return Err("Invalid iv len".into());
    }
    let key = GenericArray::clone_from_slice(key);
    let cipher = Aes128::new(&key);
    let chunks: Vec<&[u8]> = pt.chunks(chunk_size).collect();
    let mut previous_ct = iv.to_vec();
    let mut enc = vec![];
    for pt in chunks {
        let mut plain_text: Vec<u8> = pt.to_vec();
        if pt.len() != chunk_size {
            plain_text = pkcs7(pt, chunk_size);
        }
        let scrambled = appy_fixed(&previous_ct, &plain_text);
        let mut input = GenericArray::clone_from_slice(&scrambled);
        cipher.encrypt_block(&mut input);
        previous_ct = input.to_vec();
        enc.append(&mut previous_ct.clone());
    }
    Ok(enc)
}

pub fn decrypt_cbc(ct: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let chunk_size = key.len();
    if iv.len() != chunk_size {
        return Err("Invalid iv len".into());
    }
    let key = GenericArray::clone_from_slice(key);
    let cipher = Aes128::new(&key);
    let mut dec = vec![];
    let chunks: Vec<&[u8]> = ct.chunks(chunk_size).collect();
    let mut prev_ct = iv.to_vec();
    for ct in chunks {
        let mut block_dec = GenericArray::clone_from_slice(ct);
        cipher.decrypt_block(&mut block_dec);
        let plain_text = appy_fixed(&block_dec, &prev_ct);
        prev_ct = ct.to_vec();
        dec.append(&mut plain_text.clone());
    }
    strip_pkcs7(&dec)
    // Ok(dec)
}

pub fn encrypt_ecb(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let key = GenericArray::clone_from_slice(key);
    let cipher = Aes128::new(&key);
    let chunk_size = key.len();
    let chunks: Vec<&[u8]> = data.chunks(chunk_size).collect();
    let mut enc = vec![];
    for pt in chunks {
        let mut plain_text: Vec<u8> = pt.to_vec();
        if pt.len() != chunk_size {
            plain_text = pkcs7(pt, chunk_size);
        }
        let mut input = GenericArray::clone_from_slice(&plain_text);
        cipher.encrypt_block(&mut input);
        enc.append(&mut input.to_vec());
    }
    Ok(enc)
}

pub fn decrypt_ecb(data: &[u8], key: &[u8]) -> String {
    let key = GenericArray::clone_from_slice(key);
    let mut blocks = vec![];
    (0..data.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(&data[x..x + 16]));
    });

    let cipher = Aes128::new(&key);
    cipher.decrypt_blocks(&mut blocks);

    blocks.iter().flatten().map(|&x| x as char).collect()
}

#[test]
pub fn test_cbc() {
    let key = "YELLOW SUBMARINE".as_bytes();
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed auctor aliquet turpis eget interdum. Vivamus quis malesuada ante, ac dictum ante. Sed varius risus non fermentum scelerisque. Donec sodales commodo aliquet. Etiam ac ex eget purus posuere pretium vitae quis sapien. Morbi pretium bibendum pellentesque. Sed ut orci vel ex laoreet cursus nec eget nunc. Duis sit amet nulla ex. Sed ac nulla posuere, viverra magna non, sollicitudin augue. Praesent vitae augue mi.".as_bytes();
    let iv = "Believe in Magic".as_bytes();
    let enc = encrypt_cbc(text, key, iv).unwrap();
    let dec = decrypt_cbc(&enc, key, iv).unwrap();
    assert_eq!(text, dec);
}

pub fn has_duplicates(ct: &[u8], block_size: usize) -> usize {
    let mut same = 0;
    let n_chunks = ct.len() / block_size;
    for i1 in 0..n_chunks {
        let chunk1 = &ct[block_size * i1..block_size * (i1 + 1)];
        for i2 in 0..n_chunks {
            if i1 == i2 {
                continue;
            };
            let chunk2 = &ct[block_size * i2..block_size * (i2 + 1)];
            if chunk1 == chunk2 {
                same += 1;
            }
        }
    }
    same
}

pub fn pkcs7(s: &[u8], sz: usize) -> Vec<u8> {
    let padding = sz as u8 - (s.len() % sz) as u8;
    let mut res = s.to_vec();
    for _ in 0..padding {
        res.push(padding);
    }
    res
}

pub fn strip_pkcs7(s: &[u8]) -> Result<Vec<u8>> {
    let last = s[s.len() - 1] as usize;
    if last == 0 || (s.len() < last) || s[s.len() - last..].windows(2).any(|w| w[0] != w[1]) {
        return Err("Invalid padding".into());
    }

    Ok(s[0..(s.len() - last)].to_vec())
}

#[test]
pub fn test_padding() {
    let data = "YELLOW SUBMARINE";
    let padded_data = pkcs7(data.as_bytes(), 20);
    let unpadded_data = String::from_utf8(strip_pkcs7(&padded_data).unwrap()).unwrap();
    assert_eq!(padded_data, "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes());
    assert_eq!(unpadded_data, data);
}
