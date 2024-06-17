pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.len() != b.len() {
        panic!("cannot xor different lenght buffer!")
    } else {
        a.into_iter()
            .zip(b.into_iter())
            .map(|(x, y)| x ^ y)
            .collect()
    }
}
