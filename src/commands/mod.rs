mod day1;

pub use day1::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Subcommands {
  Day1(Day1),
}

impl Subcommands {
  pub fn execute(&self) {
    match self {
      Subcommands::Day1(day1) => day1.execute(),
    }
  }
}
