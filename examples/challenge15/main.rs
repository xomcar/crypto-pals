use crypto_bros::aes::try_strip_pkcs7;

pub fn main() {
    let data1 = b"ICE ICE BABY\x01\x02\x03\x04";
    let data2 = b"ICE ICE BABY\x05\x05\x05\x05";
    let data3 = b"ICE ICE BABY\x04\x04\x04\x04";

    assert_eq!(
        try_strip_pkcs7(data1).unwrap_err().as_ref().to_string(),
        "invalid padding"
    );
    assert_eq!(
        try_strip_pkcs7(data2).unwrap_err().as_ref().to_string(),
        "invalid padding"
    );
    assert_eq!(try_strip_pkcs7(data3).unwrap(), b"ICE ICE BABY");

    println!("PKCS7 strip works ok!")
}
