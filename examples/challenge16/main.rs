use crypto_bros::{aes, error::Result, xor::apply_fixed};
use rand::Rng;

pub fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    let prepend = b"comment1=cooking%20MCs;userdata=";
    let append = b";comment2=%20like%20a%20pound%20of%20bacon";
    let enc_fun = |data: &[u8]| {
        let mut total = prepend.to_vec();
        let mut escaped = escape_chars(&data);
        total.append(&mut escaped);
        total.append(&mut append.to_vec());
        aes::encrypt_cbc(&total, &key, &iv)
    };
    let dec_fun = |data: &[u8]| aes::decrypt_cbc(&data, &key, &iv);

    let target_token = b";admin=true";
    let data = b"_".repeat(32);
    let enc = enc_fun(&data)?;
    let old_dec = dec_fun(&enc)?;
    let start_index = 32 + data.len() - target_token.len();
    let end_index = start_index + target_token.len();
    let subs = apply_fixed(target_token, &old_dec[start_index..end_index])?;
    let mut attack_vector = vec![0; enc.len()];
    attack_vector[start_index - 16..end_index - 16].copy_from_slice(&subs);
    let new_enc = apply_fixed(&enc, &attack_vector)?;
    let new_dec = dec_fun(&new_enc)?;

    assert_eq!(check_admin(&new_dec, target_token), true);

    println!("Input: {}", String::from_utf8(data)?);
    println!(
        "Decoded output (original): \n{}",
        String::from_utf8(old_dec)?
    );

    println!(
        "Decoded output (cracked): \n{}",
        String::from_utf8_lossy(&new_dec)
            .chars()
            .map(|c| if c.is_control() || !c.is_ascii() {
                '*'
            } else {
                c
            })
            .collect::<String>()
    );
    println!("You cracked the admin role!");
    Ok(())
}

fn check_admin(data: &[u8], target: &[u8]) -> bool {
    data.windows(target.len()).any(|s| s == target)
}

fn escape_chars(data: &[u8]) -> Vec<u8> {
    let mut escaped = vec![];
    for &d in data {
        if d == b'=' {
            escaped.extend_from_slice(b"%3D");
        } else if d == b';' {
            escaped.extend_from_slice(b"%3B");
        } else {
            escaped.push(d);
        }
    }
    escaped
}

fn _print_str(data: &[u8], alignment: usize) {
    println!();
    let mut i = 0;
    for d in data {
        let d = *d as char;
        let c: char;
        if d.is_ascii() && !d.is_control() {
            c = d
        } else {
            c = '*'
        }
        print!("{}", c);
        i += 1;
        if i % alignment == 0 {
            println!();
        }
    }
    println!();
    println!();
}
