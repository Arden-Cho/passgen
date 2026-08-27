mod args;
mod generators;
mod utils;
mod wordlist;

use clap::Parser;

use crate::{args::CliArgs, generators::generate, utils::log};

fn main() {
    let args = CliArgs::parse().normalize();
    if args.entropy {
        let entropy = args.get_entropy();
        log(
            &"entropy",
            &format!(
                "{0:.2} bits - this could indicate a {1} password",
                entropy.0, entropy.1.0
            ),
        );
    }
    println!("{}", generate(&args));
}
