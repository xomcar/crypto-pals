use crate::xor::{self, fixed_xor};

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

pub fn get_english_lang_score(s: &[u8]) -> f32 {
    let mut freqs = [0; 256];
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

pub fn crack_single_byte_xor_slow(input: &[u8]) -> (Vec<u8>, u8) {
    let mut best_guess = vec![];
    let mut best_score = f32::MAX;
    let mut key_guess: u8 = 0;
    for key in 0u8..u8::MAX {
        let secret = vec![key; input.len()];
        let decrypted = fixed_xor(&secret, &input);
        let score = get_english_lang_score(&decrypted);
        if score < best_score {
            best_score = score;
            key_guess = key;
            best_guess = decrypted;
        }
    }
    (best_guess, key_guess)
}

pub fn crack_single_byte_xor(input: &[u8]) -> (Vec<u8>, u8) {
    let mut freqs: Vec<f32> = vec![];
    for letter in 0..256 {
        freqs
            .push(input.iter().filter(|&c| *c == letter as u8).count() as f32 / input.len() as f32);
    }

    let mut best_guess = 0u8;
    let mut best_score = f32::MAX;

    for candidate in 0..=255 {
        let mut score = 0.0f32;
        for (letter, exp_freq) in freqs.iter().enumerate() {
            score += f32::abs(exp_freq - LETTER_FREQUENCIES[letter as usize ^ candidate as usize])
        }
        if score < best_score {
            best_score = score;
            best_guess = candidate;
        }
    }

    let key = vec![best_guess; input.len()];
    let unenc = fixed_xor(input, &key);

    (unenc, best_guess)
}

pub fn hamming_dist(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());
    let mut dist = 0;
    for (c1, c2) in a.into_iter().zip(b) {
        let t = c1 ^ c2;
        dist += u8::count_ones(t);
        // for i in 0..8 {
        //     dist += (t >> i & 0x01) as u32;
        // }
    }
    dist
}

#[test]
pub fn test_hamming() {
    let dist = hamming_dist("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
    assert_eq!(dist, 37);
}
