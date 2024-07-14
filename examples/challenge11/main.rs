// An ECB/CBC detection oracle
use crypto_bros::aes::{count_duplicates, enc_oracle, AESType};
use crypto_bros::bench;
use crypto_bros::error::Result;
use rand::Rng;

pub fn solve(n_tries: usize) -> Result<()> {
    let mut rng = rand::thread_rng();
    let plain_text = "a".repeat(64);
    for _ in 0..n_tries {
        let mut enc_kind: AESType = AESType::ECB;
        if rng.gen_bool(0.5) {
            enc_kind = AESType::CBC
        };
        let enc_text = enc_oracle(&mut rng, &plain_text.as_bytes(), enc_kind)?;
        if count_duplicates(&enc_text, 16) > 0 {
            if enc_kind != AESType::ECB {
                return Err(format!("wrong prediction! expected ECB got {}", enc_kind).into());
            }
        } else {
            if enc_kind != AESType::CBC {
                return Err(format!("wrong prediction! expected CBC got {}", enc_kind).into());
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let n_tries = 100_000;
    let (result, duration) = bench!(solve(n_tries));
    match result {
        Ok(ok) => {
            println!(
                "predicted {} encryptions types in {}ms",
                n_tries,
                duration.as_millis(),
            );
            Ok(ok)
        }
        Err(e) => Err(e),
    }
}
