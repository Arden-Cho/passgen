use std::process::exit;

pub fn log(msg: &str) {
    eprintln!("passgen: fatal: {msg}");
}

/// Terminate the program with an error message and an exit code of 1.
pub fn fatal(msg: &str) -> ! {
    eprintln!("passgen: fatal: {msg}");
    exit(1)
}
