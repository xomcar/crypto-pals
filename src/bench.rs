use crate::error::Result;
use std::time::Instant;
pub fn time(f: &dyn Fn() -> Result<()>) -> Result<()> {
    let start = Instant::now();
    f()?;
    let end = Instant::now();
    println!("took: {}ms", (end - start).as_millis());
    Ok(())
}
