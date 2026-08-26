use std::{process::exit, ops::Range};

pub fn log(msg: &str) {
    eprintln!("passgen: warn: {msg}");
}

pub fn fatal(msg: &str) -> ! {
    eprintln!("passgen: fatal: {msg}");
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
