mod day1;

pub use day1::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Subcommands {
  Day1(Day1),
}

impl Subcommands {
  pub fn execute(&self, part_two: bool, sample: bool) {
    match self {
      Subcommands::Day1(day1) => day1.execute(part_two, sample),
    }
  }
}

pub trait AoCCommand {
  fn execute(&self, part_two: bool, sample: bool);
  fn execute_part_one(&self, input: &str);
  fn execute_part_two(&self, input: &str);
}
