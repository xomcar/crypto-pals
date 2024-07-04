use crypto_bros::{hex, xor};

const STANZA: &str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";

const ENCRYPTED_SOLUTION: &str =
    "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

const KEY: &str = "ICE";
fn main() {
    let enc = xor::repeating_xor(STANZA.as_bytes(), KEY.as_bytes());
    let hex_enc = hex::encode(&enc);
    assert_eq!(hex_enc, ENCRYPTED_SOLUTION);
    println!("{}", hex_enc);
}
