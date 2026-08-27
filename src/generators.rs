use crate::{
    args::AppArgs, utils::{log_warn, random_range_usize}, wordlist::{self, WORDS},
};

pub fn generate(args: &AppArgs) -> String {
    if args.get_entropy().1.1 <= 1 {
        log_warn("this is a weak password");
    }
    if args.passphrase {
        generate_passphrase(args)
    } else {
        generate_password(args)
    }
}

fn generate_passphrase(args: &AppArgs) -> String {
    let mut password = String::new();
    for _ in 0..args.length {
        let word = wordlist::WORDS[random_range_usize(0..WORDS.len())];
        password += args
            .upper
            .then(|| word[0..1].to_ascii_uppercase() + &word[1..])
            .as_deref()
            .unwrap_or(word);
    }
    password
}

fn generate_password(args: &AppArgs) -> String {
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
