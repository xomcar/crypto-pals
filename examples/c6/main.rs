use std::fs;

use crypto_bros::{base64, cypher, xor};

pub fn main() {
    let mut base64_encoded_data = fs::read_to_string("data/6.txt").expect("File not found");
    base64_encoded_data.retain(|c| !c.is_ascii_whitespace());
    let cypher_text_data = base64::decode(&base64_encoded_data).unwrap();
    let key_sizes = cypher::guess_keysize(&cypher_text_data, 1);
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
            let (_, key_guess) = cypher::crack_single_byte_xor(&picked);
            key.push(key_guess);
        }
        let unenc = xor::repeating_xor(&cypher_text_data, &key);
        println!(
            "================ Guessed key size: {} =====================",
            ksz
        );
        let key_str = String::from_utf8(key).unwrap();
        println!("Guessed key: {}", key_str);
        println!("===========================================================");
        println!("{}", String::from_utf8(unenc).unwrap());

        assert_eq!(key_str, "Terminator X: Bring the noise")
    }
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
