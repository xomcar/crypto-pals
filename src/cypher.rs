use crate::xor::fixed_xor;
use itertools::Itertools;
use std::cmp::min;

const LETTER_FREQUENCIES: [f32; 256] = {
    let mut frequencies = [0.0; 256];
    frequencies[b'a' as usize] = 8.4966 / 100.0;
    frequencies[b'b' as usize] = 2.0720 / 100.0;
    frequencies[b'c' as usize] = 4.5388 / 100.0;
    frequencies[b'd' as usize] = 3.3844 / 100.0;
    frequencies[b'e' as usize] = 11.160 / 100.0;
    frequencies[b'f' as usize] = 1.8121 / 100.0;
    frequencies[b'g' as usize] = 2.4705 / 100.0;
    frequencies[b'h' as usize] = 3.0034 / 100.0;
    frequencies[b'i' as usize] = 7.5448 / 100.0;
    frequencies[b'j' as usize] = 0.1965 / 100.0;
    frequencies[b'k' as usize] = 1.1016 / 100.0;
    frequencies[b'l' as usize] = 5.4893 / 100.0;
    frequencies[b'm' as usize] = 3.0129 / 100.0;
    frequencies[b'n' as usize] = 6.6544 / 100.0;
    frequencies[b'o' as usize] = 7.1635 / 100.0;
    frequencies[b'p' as usize] = 3.1671 / 100.0;
    frequencies[b'q' as usize] = 0.1962 / 100.0;
    frequencies[b'r' as usize] = 7.5809 / 100.0;
    frequencies[b's' as usize] = 5.7351 / 100.0;
    frequencies[b't' as usize] = 6.9509 / 100.0;
    frequencies[b'u' as usize] = 3.6308 / 100.0;
    frequencies[b'v' as usize] = 1.0074 / 100.0;
    frequencies[b'w' as usize] = 1.2899 / 100.0;
    frequencies[b'x' as usize] = 0.2902 / 100.0;
    frequencies[b'y' as usize] = 1.7779 / 100.0;
    frequencies[b'z' as usize] = 0.2722 / 100.0;

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

pub fn guess_keysize(data: &[u8], max_guesses: usize) -> Vec<(f32, usize)> {
    let mut guess_map: Vec<(f32, usize)> = vec![];
    let max_key_size = min(40, data.len() / 4);
    for key_size in 2..max_key_size {
        let first = &data[0..1 * key_size];
        let second = &data[1 * key_size..2 * key_size];
        let third = &data[2 * key_size..3 * key_size];
        let fourth = &data[3 * key_size..4 * key_size];
        let chunks = [first, second, third, fourth];
        let elements = chunks.iter().combinations(2);
        let mut combs = 0;
        let dist: u32 = elements
            .into_iter()
            .map(|v| {
                combs += 1;
                hamming_dist(v[0], v[1])
            })
            .sum();
        let norm_dist = (dist / combs) as f32 / key_size as f32;
        guess_map.push((norm_dist, key_size));
    }
    guess_map.sort_by(|a, b| a.0.total_cmp(&b.0));
    //println!("{:?}", guess_map);
    guess_map[0..max_guesses]
        .into_iter()
        .map(|(conf, sz)| (*conf, *sz))
        .collect()
}

#[test]
pub fn test_hamming() {
    let dist = hamming_dist("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
    assert_eq!(dist, 37);
}
