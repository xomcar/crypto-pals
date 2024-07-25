// Implement repeating-key XOR
use crypto_bros::{error::Result, hex, xor};

fn main() -> Result<()> {
    let input_str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let expected_enc_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let key = "ICE";
    let enc = xor::apply_repeating(input_str.as_bytes(), key.as_bytes())?;
    let hex_enc = hex::encode(&enc);

    assert_eq!(hex_enc, expected_enc_hex);

    println!(
        "input text:\n\t{}\nusing key:\n\t{}\nwith rotating xor cypher:\n\t{}",
        input_str, key, hex_enc
    );
    Ok(())
}
