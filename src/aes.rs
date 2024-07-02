
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, NewBlockCipher};
use aes::Aes128;


pub fn decrypt(data : &[u8], key : &[u8]) -> String {
    let key = GenericArray::clone_from_slice(key);
    let mut blocks = vec![];
    (0..data.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(&data[x..x+16]));
    });

    let cipher = Aes128::new(&key);
    cipher.decrypt_blocks(&mut blocks);

    blocks.iter().flatten().map(|&x| x as char).collect()
}

pub fn has_duplicates(ct : &[u8], block_size : usize) -> usize {
    let mut same = 0;
        let n_chunks = ct.len() / block_size;
        for i1 in 0..n_chunks {
            let chunk1 = &ct[block_size*i1..block_size*(i1+1)];
            for i2 in 0..n_chunks {
                if i1 == i2 {continue};
                let chunk2 = &ct[block_size*i2..block_size*(i2+1)];
                if chunk1 == chunk2 {
                    same += 1;
                }
            }        
        }
    same
}

pub fn pad(s :&str, sz : usize) -> String {
    let padding = (sz - s.len()) as u8;
    let mut res = s.to_string();
    for _ in 0..padding {
        res.push(padding as char);
    }
    res
}

#[test]
pub fn test_padding() {
    let data = "YELLOW SUBMARINE";
    let padded_data = pad(data, 20);
    assert_eq!(padded_data, "YELLOW SUBMARINE\x04\x04\x04\x04");
}