// Fixed XOR
use crypto_bros::error::Result;
use crypto_bros::{hex, xor};
pub fn main() -> Result<()> {
    let hex_input = "1c0111001f010100061a024b53535009181c";
    let hex_key = "686974207468652062756c6c277320657965";
    let hex_expected_output = "746865206b696420646f6e277420706c6179";
    let plain_input = hex::decode(&hex_input)?;
    let plain_key = hex::decode(&hex_key)?;
    let plain_expected_output = hex::decode(&hex_expected_output)?;

    let xord = xor::appy_fixed(&plain_input, &plain_key);
    assert_eq!(xord, plain_expected_output);

    let xord_str = String::from_utf8(xord)?;
    let input_str = String::from_utf8(plain_input)?;
    let key_str = String::from_utf8(plain_key)?;
    println!(
        "hex input:\n\t{}\nxor'ed with:\n\t{}\nresults in:\n\t{}",
        input_str, key_str, xord_str,
    );
    Ok(())
}
