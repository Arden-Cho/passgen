mod args;
mod generators;
mod utils;

use clap::Parser;
use colored::Colorize;

use crate::{args::Args, generators::generate_password, utils::{calc_entropy, log}};

fn main() {
    let args = Args::parse().normalize();
    if args.entropy {
        let entropy = calc_entropy(args.length.into(), args.get_charset().len() as f64);
        log(&"entropy", &format!("{entropy:.2} bits - this could indicate a {} password", {
            match entropy {
                0.0..35.0 => {
                    "very weak".red()
                }
                35.0..59.0 => {
                    "weak".yellow()
                }
                59.0..79.0 => {
                    "good".blue()
                }
                79.0.. => {
                    "strong".green()
                }
                _ => {
                    panic!("BUG: calc_entropy returned a negative float")
                }
            }
        }));
    }
    println!("{}", generate_password(&args));
}
