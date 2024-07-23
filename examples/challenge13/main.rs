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

    // check text len
    let text_len = aes::find_text_len(&mut user_enc, block_len)?;
    println!("Text length: {}", text_len);

    let mail = "aaron@german.com";
    let enc_cookie = user_enc(mail.as_bytes())?;
    let profile = user_dec(&enc_cookie)?;
    println!("Enc cookie: {:?}\n{:?}", &enc_cookie, &profile);
    Ok(())
}
