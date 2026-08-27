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
pub struct Args {
    /// Include uppercase characters
    #[arg(short, long)]
    pub upper: bool,

    /// Include lowercase characters
    #[arg(short, long)]
    pub lower: bool,

    /// Include numbers
    #[arg(short, long)]
    pub numbers: bool,

    /// Include special characters [!@#$%^&*]
    #[arg(short, long)]
    pub special: bool,

    /// Exclude ambiguous characters [l1I0O]
    #[arg(short = 'a', long)]
    pub no_ambiguous: bool,

    /// The length of the password
    #[arg(short = 'L', long, default_value_t = 16)]
    pub length: u8,

    /// Output the ESTIMATED entropy of the password
    ///
    /// This could give you a rough idea of how strong the password generated is.
    #[arg(short, long)]
    pub entropy: bool,
}

impl Args {
    fn active_group_count(&self) -> u8 {
        self.upper as u8 + self.lower as u8 + self.numbers as u8 + self.special as u8
    }

    fn validate(&self) {
        if self.active_group_count() > self.length {
            fatal(
                "generating a password of this length is impossible with the chosen groups (see --help)",
            );
        }
        if self.length < 8 {
            log_warn("weak password - a password with a length < 8 could be easily brute-forced");
        }
    }

    fn apply_defaults(&mut self) {
        if self.active_group_count() == 0 {
            self.upper = true;
            self.lower = true;
            self.numbers = true;
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

    pub fn normalize(mut self) -> Self {
        self.apply_defaults();
        self.validate();
        self
    }
}
