use crypto_bros::{
    aes::{self, decrypt_ecb, encrypt_ecb},
    cookie,
    error::Result,
};
pub fn main() -> Result<()> {
    solve()
}

fn solve() -> Result<()> {
    let key = "YELLOW SUBMARINE".to_owned();
    let mut enc_func = |data: &[u8]| encrypt_ecb(&data, key.as_bytes());
    let mut dec_func = |data: &[u8]| decrypt_ecb(&data, key.as_bytes());
    let mut user_enc = |mail_data: &[u8]| {
        let mail = String::from_utf8(mail_data.to_vec())?;
        cookie::encrypt_user(mail.as_str(), &mut enc_func)
    };
    let mut user_dec = |enc_mail_data: &[u8]| cookie::decrypt_user(enc_mail_data, &mut dec_func);

    // check block len
    let block_len = aes::find_block_len(&mut user_enc, 40)?;
    println!("Block len :{}", block_len);
    assert_eq!(16, block_len);

    // First attack:
    // email=aaaaaaaaaa
    // admin00000000000
    // &uid=4233230557&
    // role=user

    let first_email = "a".repeat(block_len - "email=".len());
    let second_email = b"a@b.c";
    let second_row = String::from_utf8(aes::pad_pkcs7(b"admin", block_len))?;
    let mail = format!("{}{}", &first_email, &second_row);
    let enc_cookie = user_enc(mail.as_bytes())?;
    let user_profile = user_dec(&enc_cookie)?;
    let attack_block = enc_cookie[16..32].to_vec();

    // Second attack
    // email=a@b.c&uid=
    // 4233230557&role=
    // admin

    let enc_cookie = user_enc(second_email)?;
    let cracked_cookie: Vec<u8> = [enc_cookie[..32].to_vec(), attack_block].concat();
    let admin_profile = user_dec(&cracked_cookie)?;
    assert_eq!(admin_profile.role, cookie::Role::Admin);

    println!("From user profile: {:?}", user_profile);
    println!("Got admin access: {:?}", admin_profile);
    Ok(())
}
