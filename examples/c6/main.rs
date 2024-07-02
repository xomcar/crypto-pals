use crypto_bros::{
    base64,
    cypher::{crack_single_byte_xor, crack_single_byte_xor_slow, hamming_dist},
    xor::repeating_xor,
};
use itertools::Itertools;
use std::{cmp::min, fs};

pub fn main() {
    let mut base64_encoded_data = fs::read_to_string("data/6.txt").expect("File not found");
    base64_encoded_data.retain(|c| !c.is_ascii_whitespace());
    let cypher_text_data = base64::decode(&base64_encoded_data).unwrap();
    let key_sizes = guess_keysize(&cypher_text_data, 1);
    for (_, ksz) in key_sizes {
        let mut key: Vec<u8> = Vec::with_capacity(ksz);
        for i in 0..ksz {
            let picked: Vec<u8> = cypher_text_data
                .clone()
                .into_iter()
                .skip(i)
                .step_by(ksz)
                .collect();
            // println!("{:?}\n{:?}\n", &picked[0..35], &cypher_text_data[0..35]);
            let (_, key_guess) = crack_single_byte_xor(&picked);
            key.push(key_guess);
        }
        let unenc = repeating_xor(&cypher_text_data, &key);
        println!(
            "================ Guessed key size: {} =====================",
            ksz
        );
        let key_str = String::from_utf8(key).unwrap();
        println!("Guessed key: {}", key_str);
        println!("===========================================================");
        println!(
            "Unencrypted text:\n------------\n{}",
            String::from_utf8(unenc).unwrap()
        );
        println!("=================================================");
        assert_eq!(key_str, "Terminator X: Bring the noise")
    }
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
pub fn test_keysize() {
    let plain_txt = " Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed auctor aliquet turpis eget interdum. Vivamus quis malesuada ante, ac dictum ante. Sed varius risus non fermentum scelerisque. Donec sodales commodo aliquet. Etiam ac ex eget purus posuere pretium vitae quis sapien. Morbi pretium bibendum pellentesque. Sed ut orci vel ex laoreet cursus nec eget nunc. Duis sit amet nulla ex. Sed ac nulla posuere, viverra magna non, sollicitudin augue. Praesent vitae augue mi. Duis et orci ligula.

Maecenas finibus sed magna in eleifend. Aliquam non lorem et tortor placerat porta et in augue. Pellentesque faucibus risus eget vehicula facilisis. Suspendisse et arcu nec mauris consequat interdum. Aenean eros neque, pulvinar ac dapibus in, sagittis in sapien. Sed nec augue et quam lacinia tristique quis sed lacus. Nam ac augue dui. In ornare tincidunt placerat. In mattis enim elit, nec dapibus lectus pellentesque ut. Quisque congue non risus sed luctus.

Vivamus gravida pretium malesuada. Aenean efficitur sollicitudin libero, eget elementum dolor auctor quis. Sed dignissim augue id ex tempor, vitae viverra nisi dictum. Aenean imperdiet, augue vitae aliquet pharetra, odio lacus aliquet urna, sit amet eleifend nisl nisl in eros. Mauris eget sapien fermentum, sagittis elit eget, porttitor lectus. Vestibulum molestie erat eu est sodales, vel rutrum nisi molestie. Ut purus massa, semper ac sem non, venenatis egestas diam. Suspendisse a dolor dignissim, tempor nulla eget, fringilla nulla. Aenean rhoncus, ex et scelerisque tempus, justo mi dapibus arcu, ut fermentum lacus augue vitae nisi. Sed quis ligula dolor. Donec nisl enim, blandit vel enim eget, sollicitudin tincidunt risus. Morbi convallis a dui id pharetra. Sed accumsan orci vel nulla commodo, ut ullamcorper velit consectetur. ";
    let key = "hunter2";
    let enc = xor::repeating_xor(plain_txt, key);
    let est_key_sizes = guess_keysize(&enc, 10);
    let mut found = false;
    for (_, sz) in est_key_sizes {
        if key.len() == sz {
            found = true;
            break;
        }
    }
    assert!(found);
}
