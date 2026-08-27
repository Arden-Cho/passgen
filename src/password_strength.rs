use std::fmt::{Display, Formatter, Result};

use colored::Colorize;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Good,
    Strong,
}

impl Display for PasswordStrength {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                PasswordStrength::VeryWeak => "very weak".red().bold(),
                PasswordStrength::Weak => "weak".yellow().bold(),
                PasswordStrength::Good => "good".blue().bold(),
                PasswordStrength::Strong => "strong".green().bold(),
            }
        )
    }
}
