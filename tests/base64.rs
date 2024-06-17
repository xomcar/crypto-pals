pub const EXAMPLE_INPUT : &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
pub const EXAMPLE_OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

#[test]
fn check_base64_algo() {
    use crypto_bros::base64;
    use crypto_bros::hex;
    let input_string = hex::decode_hex(EXAMPLE_INPUT).unwrap();
    let result = base64::to_base64(input_string.as_slice());
    assert_eq!(result, EXAMPLE_OUTPUT)
}
