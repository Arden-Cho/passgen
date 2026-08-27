use std::num::{NonZero, NonZeroU8};

use clap::Parser;
use colored::{ColoredString, Colorize};

use crate::utils::{fatal, log_warn};

/// A password generator
///
/// If no option is specified the program is run as below
///
/// passgen -ulnL 16
#[derive(Parser, Clone)]
#[command(version, about)]
pub struct CliArgs {
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

    /// Exclude ambiguous characters [l1I0O]
    #[arg(short = 'a', long, conflicts_with = "passphrase")]
    no_ambiguous: bool,

    /// Generate a diceware-like passphrase instead of a password
    #[arg(short, long)]
    passphrase: bool,

    /// The length of the password
    #[arg(short = 'L', long)]
    length: Option<NonZeroU8>,

    /// Output the ESTIMATED entropy of the password
    ///
    /// This could give you a rough idea of how strong the password generated is.
    #[arg(short, long)]
    entropy: bool,
}

impl CliArgs {
    fn active_group_count(&self) -> u8 {
        self.upper as u8 + self.lower as u8 + self.numbers as u8 + self.special as u8
    }

    pub fn normalize(self) -> AppArgs {
        AppArgs::from_cli(self)
    }
}

#[derive(Clone)]
pub struct AppArgs {
    pub upper: bool,
    pub lower: bool,
    pub numbers: bool,
    pub special: bool,
    pub no_ambiguous: bool,
    pub passphrase: bool,
    pub length: u8,
    pub entropy: bool,
}

impl AppArgs {
    const DEFAULT_PASSPHRASE_LENGTH: NonZeroU8 = NonZero::new(6).unwrap();
    const DEFAULT_PASSWORD_LENGTH: NonZeroU8 = NonZero::new(16).unwrap();

    pub fn from_cli(mut args: CliArgs) -> AppArgs {
        let length = args
            .length
            .unwrap_or_else(|| {
                if args.passphrase {
                    AppArgs::DEFAULT_PASSPHRASE_LENGTH
                } else {
                    AppArgs::DEFAULT_PASSWORD_LENGTH
                }
            })
            .get();
        if args.active_group_count() == 0 {
            args.upper = true;
            args.lower = true;
            args.numbers = true;
        }
        if args.active_group_count() > length {
            fatal(
                "generating a password of this length is impossible with the chosen groups (see --help)",
            );
        }
        AppArgs {
            upper: args.upper,
            lower: args.lower,
            numbers: args.numbers,
            special: args.special,
            no_ambiguous: args.no_ambiguous,
            passphrase: args.passphrase,
            length: length,
            entropy: args.entropy,
        }
    }

    pub fn get_charset(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(70);
        if self.upper {
            bytes.extend_from_slice(b"QWERTYUPASDFGHJKLZXCVBNM");
            if !self.no_ambiguous {
                bytes.extend_from_slice(b"IO");
            }
        }
        if self.lower {
            bytes.extend_from_slice(b"qwertyuiopasdfghjkzxcvbnm");
            if !self.no_ambiguous {
                bytes.extend_from_slice(b"l");
            }
        }
        if self.numbers {
            bytes.extend_from_slice(b"23456789");
            if !self.no_ambiguous {
                bytes.extend_from_slice(b"01");
            }
        }
        if self.special {
            bytes.extend_from_slice(b"!@#$%^&*");
        }
        bytes
    }

    pub fn get_entropy(&self) -> (f64, ColoredString) {
        let entropy = (self.get_charset().len() as f64).log2() * (self.length as f64);
        (
            entropy,
            match entropy {
                0.0..35.0 => "very weak".red().bold(),
                35.0..59.0 => "weak".yellow().bold(),
                59.0..79.0 => "good".blue().bold(),
                79.0.. => "strong".green().bold(),
                _ => {
                    unreachable!("BUG: Args::get_entropy")
                }
            },
        )
    }
}
