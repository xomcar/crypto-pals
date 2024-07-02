
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