mod utils;

use clap::Parser;
use rand::RngExt;
use utils::fatal;

use crate::utils::log;

#[derive(Parser, Clone)]
#[command(version, about)]
struct Args {
    /// Include uppercase characters
    #[arg(short, long)]
    upper: bool,

    /// Include lowercase characters
    #[arg(short, long)]
    lower: bool,

    /// Include numbers
    #[arg(short, long)]
    numbers: bool,

    /// Include special characters [!@#$%^&*]
    #[arg(short, long)]
    special: bool,
    
    /// The length of the password
    #[arg(short = 'L', long, default_value_t = 16)]
    length: u8,
}

fn validate_args(args: &Args) {
    let active_group_count =
        args.upper as u8 + args.lower as u8 + args.numbers as u8 + args.special as u8;
    if active_group_count == 0 {
        fatal("must choose at least one group of characters (see --help)");
    }
    if active_group_count > args.length {
        fatal("generating a password of this length is impossible with the chosen groups");
    }
    if args.length < 8 {
        log("weak password - a password with a length < 8 could be easily brute-forced");
    }
}

fn generate_password(args: &Args) -> String {
    let chars = {
        let mut s = String::new();
        if args.upper {
            s += "QWERTYUIOPASDFGHJKLZXCVBNM";
        }
        if args.lower {
            s += "qwertyuiopasdfghjklzxcvbnm";
        }
        if args.numbers {
            s += "1234567890";
        }
        if args.special {
            s += "!@#$%^&*";
        }
        s.into_bytes()
    };
    let mut rng = rand::rng();
    let mut password: String;
    loop {
        let mut args = args.clone();
        password = (0..args.length)
            .map(|_| {
                let idx = rng.random_range(0..chars.len());
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
            })
            .collect();
        if !(args.upper || args.lower || args.numbers || args.special) {
            break;
        }
    }
    password
}

fn main() {
    let args = Args::parse();
    validate_args(&args);
    println!("{}", generate_password(&args));
}
