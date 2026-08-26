use std::{process::exit, ops::Range};

use colored::Colorize;

pub fn log(prefix: &dyn std::fmt::Display, msg: &dyn std::fmt::Display) {
    eprintln!("passgen: {prefix}: {msg}");
}

pub fn log_warn(msg: &str) {
    log(&"warn".yellow().bold(), &msg);
}

pub fn fatal(msg: &str) -> ! {
    log(&"fatal".red().bold(), &msg);
    exit(1)
}

pub fn random_range_usize(range: Range<usize>) -> usize {
    assert!(range.end > range.start);
    let threshold = usize::MAX - (range.end - range.start).wrapping_neg() % (range.end - range.start);
    loop {
        let mut bytes = [0u8; std::mem::size_of::<usize>()];
        getrandom::fill(&mut bytes).is_err().then(|| fatal("failed to obtain secure randomness"));
        let r = usize::from_le_bytes(bytes);
        if r <= threshold {
            return range.start + r % (range.end - range.start);
        }
    }
}


pub fn calc_entropy(length: f64, pool_size: f64) -> f64 {
    pool_size.log2() * length
}