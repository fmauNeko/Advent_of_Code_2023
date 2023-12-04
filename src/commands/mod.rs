mod day1;
mod day2;

pub use day1::*;
pub use day2::*;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Subcommands {
  Day1(Day1),
  Day2(Day2),
}

impl Subcommands {
  pub fn execute(&self, part_two: bool) {
    match self {
      Subcommands::Day1(day1) => day1.execute(part_two),
      Subcommands::Day2(day2) => day2.execute(part_two),
    }
  }
}

pub trait AoCCommand {
  fn execute(&self, part_two: bool);
  fn execute_part_one(&self, input: &str) -> u32;
  fn execute_part_two(&self, input: &str) -> u32;
}
