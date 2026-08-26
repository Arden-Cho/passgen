use crate::{args::Args, utils::random_range_usize};

pub fn generate_password(args: &Args) -> String {
    let chars = args.get_charset();
    let mut password: String = String::with_capacity(args.length as usize);
    loop {
        let mut args = args.clone();
        password.extend((0..args.length).map(|_| {
            let idx = random_range_usize(0..chars.len());
            let c = chars[idx] as char;
            if c.is_ascii_uppercase() {
                args.upper = false;
            } else if c.is_ascii_lowercase() {
                args.lower = false;
            } else if c.is_ascii_digit() {
                args.numbers = false;
            } else {
                args.special = false;
            }
            c
        }));
        if !(args.upper || args.lower || args.numbers || args.special) {
            break;
        }
        password.clear();
    }
    password
}