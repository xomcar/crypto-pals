// Implement PKCS#7 padding
use crypto_bros::aes;
use crypto_bros::error::Result;

pub fn main() -> Result<()> {
    let input_str = "YELLOW SUBMARINE";
    let expected_output_str = "YELLOW SUBMARINE\x04\x04\x04\x04";
    let padded = aes::pad_pkcs7(input_str.as_bytes(), 20);
    let padded_str = String::from_utf8(padded)?;
    assert_eq!(padded_str, expected_output_str);
    println!(
        "input:\n\t{}\npadded at 20 bytes:\n\t{}",
        input_str, padded_str
    );
    Ok(())
}
