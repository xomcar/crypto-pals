use crate::{
    aes::{DecFun, EncFun},
    error::Result,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserProfile {
    email: String,
    uid: u32,
    role: Role,
}

impl UserProfile {
    pub fn from_str(query_str: &str) -> Result<UserProfile> {
        let result = serde_qs::from_str::<UserProfile>(query_str)?;
        Ok(result)
    }

    pub fn to_str(self: &Self) -> Result<String> {
        let result = serde_qs::to_string(self)?;
        let result = urlencoding::decode(&result)?;
        Ok(result.into_owned())
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
}

fn user_profile(mail: &str) -> Result<UserProfile> {
    let mail = mail.to_string();
    if mail.contains("&") || mail.contains("=") {
        return Err("invalid character in mail".into());
    }
    let uid = rand::random::<u32>();
    let result = UserProfile {
        email: mail,
        uid,
        role: Role::User,
    };
    Ok(result)
}

pub fn encrypt_user_cookie(mail: &str, enc_func: EncFun) -> Result<Vec<u8>> {
    let profile = user_profile(mail)?;
    let profile_str = profile.to_str()?;
    enc_func(profile_str.as_bytes())
}

pub fn decrypt_user_cookie(input: &[u8], dec_func: DecFun) -> Result<UserProfile> {
    let dec = dec_func(input)?;
    UserProfile::from_str(String::from_utf8(dec)?.as_str())
}

#[test]
fn test_enc() {
    use crate::aes::{decrypt_ecb, encrypt_ecb};
    use rand::Rng;

    let mail = "mario@none.com";
    let mut key = [0u8; 16];
    rand::thread_rng().fill(&mut key);
    let mut enc_fun = |data: &[u8]| encrypt_ecb(&data, &key);
    let mut dec_fun = |data: &[u8]| decrypt_ecb(&data, &key);
    let enc = encrypt_user_cookie(mail, &mut enc_fun).unwrap();
    let dec = decrypt_user_cookie(&enc, &mut dec_fun).unwrap();
    assert_eq!(mail, dec.email);
    println!("{:?}", dec);
}

#[test]
fn test_serialize() {
    let query = "email=foo@bar.com&uid=10&role=user";
    let user = UserProfile::from_str(&query).unwrap();
    let s = user.to_str().unwrap();
    assert_eq!(query, s);
    let user2 = UserProfile::from_str(&s).unwrap();
    assert_eq!(user, user2);
}

#[test]
fn test_fail_user() {
    let query = "foo&@bar.com";
    let result = user_profile(&query);
    match result {
        Ok(_) => panic!("shouldn't be here"),
        Err(_) => {}
    }

    let query = "foo=@bar.com";
    let result = user_profile(&query);
    match result {
        Ok(_) => panic!("shouldn't be here"),
        Err(_) => {}
    }
}
