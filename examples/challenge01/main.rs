// Convert hex to base64
use crypto_bros::error::Result;
use crypto_bros::{base64, hex};
pub fn main() -> Result<()> {
    let encoded_input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let data = hex::decode(&encoded_input)?;
    let encoded_data = base64::encode(&data);
    let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(encoded_data, expected_output);

    println!(
        "input as hex:\n\t{}\ndecoded from hex:\n\t{}\nencoded as base64:\n\t{}",
        encoded_input,
        String::from_utf8(data)?,
        encoded_data
    );
    Ok(())
}
