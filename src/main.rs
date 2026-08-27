mod args;
mod generators;
mod utils;

use clap::Parser;

use crate::{args::Args, generators::generate_password, utils::log};

fn main() {
    let args = Args::parse().normalize();
    if args.entropy {
        let entropy = args.get_entropy();
        log(
            &"entropy",
            &format!(
                "{0:.2} bits - this could indicate a {1} password",
                entropy.0, entropy.1
            ),
        );
    }
    println!("{}", generate_password(&args));
}
