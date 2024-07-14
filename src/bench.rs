#[macro_export]
macro_rules! bench {
    ($arg:expr) => {{
        let start = ::std::time::Instant::now();
        let out = $arg;
        let duration = start.elapsed();
        (out, duration)
    }};
}
