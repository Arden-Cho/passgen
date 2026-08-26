mod args;
mod generators;
mod utils;

use clap::Parser;

use crate::{args::Args, generators::generate_password};

fn main() {
    let args = Args::parse().normalize();
    println!("{}", generate_password(&args));
}
