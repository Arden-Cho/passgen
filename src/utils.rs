use std::process::exit;

pub fn log(msg: &str) {
    eprintln!("passgen: warn: {msg}");
}

pub fn fatal(msg: &str) -> ! {
    eprintln!("passgen: fatal: {msg}");
    exit(1)
}
