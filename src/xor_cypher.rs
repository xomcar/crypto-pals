use crate::{hex, hex::VecU8Ext, xor::xor};

const SIZE: usize = 256;
const ENCRYPTED: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
const LETTER_FREQUENCIES: [f32; 256] = {
    let mut frequencies = [0.0; 256];
    frequencies[b'a' as usize] = 8.4966;
    frequencies[b'b' as usize] = 2.0720;
    frequencies[b'c' as usize] = 4.5388;
    frequencies[b'd' as usize] = 3.3844;
    frequencies[b'e' as usize] = 11.1607;
    frequencies[b'f' as usize] = 1.8121;
    frequencies[b'g' as usize] = 2.4705;
    frequencies[b'h' as usize] = 3.0034;
    frequencies[b'i' as usize] = 7.5448;
    frequencies[b'j' as usize] = 0.1965;
    frequencies[b'k' as usize] = 1.1016;
    frequencies[b'l' as usize] = 5.4893;
    frequencies[b'm' as usize] = 3.0129;
    frequencies[b'n' as usize] = 6.6544;
    frequencies[b'o' as usize] = 7.1635;
    frequencies[b'p' as usize] = 3.1671;
    frequencies[b'q' as usize] = 0.1962;
    frequencies[b'r' as usize] = 7.5809;
    frequencies[b's' as usize] = 5.7351;
    frequencies[b't' as usize] = 6.9509;
    frequencies[b'u' as usize] = 3.6308;
    frequencies[b'v' as usize] = 1.0074;
    frequencies[b'w' as usize] = 1.2899;
    frequencies[b'x' as usize] = 0.2902;
    frequencies[b'y' as usize] = 1.7779;
    frequencies[b'z' as usize] = 0.2722;

    frequencies
};

pub fn find_key(input: &str) -> Vec<u8> {
    let mut map: Vec<(Vec<u8>, f32, u8)> = Vec::new();
    for key in 0u8..u8::MAX {
        let plain = hex::decode_hex(input).unwrap();
        let secret = vec![key; plain.len()];
        let decrypted = xor(&secret, &plain);
        // println!("{}", decrypted.to_ascii_string());
        let score = get_score(&decrypted);
        map.push((decrypted, score, key));
    }
    map.sort_by(|a, b| b.1.total_cmp(&a.1));
    map.pop().unwrap().0
}

pub fn get_score(s: &Vec<u8>) -> f32 {
    let mut freqs = [0; SIZE];
    for &char in s {
        freqs[char as usize] += 1;
    }
    let mut score = 0f32;
    for (i, &freq) in freqs.iter().enumerate() {
        let val = (freq as f32 / s.len() as f32) - LETTER_FREQUENCIES[i];
        score += if val < 0.0 { -val } else { val };
    }
    return score;
}

#[test]
fn test_list() {
    let guess = find_key(ENCRYPTED);
    assert_eq!(
        guess.to_ascii_string(),
        "Cooking MC's like a pound of bacon"
    )
}
