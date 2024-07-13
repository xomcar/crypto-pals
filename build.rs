extern crate proc_macro;

use quote::quote;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read as _;
use std::io::Write;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() {
    let output_path = "src/frequency.rs";
    let values = alphabet_absolute_frequency_from_file("data/shakespeare.txt")
        .expect("Failed to generate frequency from file");

    let expanded = quote! {
        pub const ENGLISH_ASCII_FREQUENCY: [f32; 256] = [
            #( #values ),*
        ];
    };

    let mut file = File::create(output_path).expect("Failed to create output file");
    write!(file, "{}", expanded).expect("Failed to write to output file");

    println!("Generated file: {}", output_path);
}

pub fn alphabet_absolute_frequency_from_file(path: &str) -> Result<[f32; 256]> {
    let mut freqs = [0.0; 256];
    let file: fs::File = fs::File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut amount = 0;
    buf_reader
        .bytes()
        .filter_map(|b| b.ok())
        .filter(|&byte| byte.is_ascii_alphabetic())
        .for_each(|byte| {
            freqs[byte as usize] += 1.0;
            amount += 1;
        });
    freqs.iter_mut().for_each(|x| *x /= amount as f32);
    Ok(freqs)
}
