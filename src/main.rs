mod args;
mod generators;
mod password_strength;
mod utils;
mod wordlist;

use clap::Parser;

use crate::{
    args::CliArgs,
    generators::generate,
    password_strength::PasswordStrength,
    utils::{log, log_warn},
};

fn main() {
    let args = CliArgs::parse().normalize();
    if args.get_entropy().1 <= PasswordStrength::Weak {
        log_warn("weak configuration, consider increasing the complexity");
    }
    if args.entropy {
        let entropy = args.get_entropy();
        log(
            &"entropy",
            &format!(
                "{0:.2} bits - {1}",
                entropy.0, entropy.1
            ),
        );
    }
    for _ in 0..args.count {
        println!("{}", generate(&args));
    }
}
