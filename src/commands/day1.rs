use clap::Parser;
use regex::Regex;

use super::AoCCommand;

#[derive(Parser)]
pub struct Day1 {}

impl AoCCommand for Day1 {
  fn execute(&self, part_two: bool) {
    let input = include_str!("../../inputs/day1.txt");
    let result;

    if part_two {
      result = self.execute_part_two(input);
    } else {
      result = self.execute_part_one(input);
    }

    println!("{}", result);
  }

  fn execute_part_one(&self, input: &str) -> u32 {
    let re = Regex::new(r"(?m)^\D*(?P<first>\d)(?:.*(?P<last>\d))?\D*$").unwrap();
    let result: u32 = re.captures_iter(input).fold(0u32, |acc, c| {
      let first = c.name("first").unwrap();
      let last = c.name("last").unwrap_or(first);

      let value_string = first.as_str().to_string() + last.as_str();
      let value = value_string.parse::<u32>().unwrap();

      acc + value
    });

    result
  }

  fn execute_part_two(&self, input: &str) -> u32 {
    let re = Regex::new(r"(?m)^.*?(?P<first>\d|one|two|three|four|five|six|seven|eight|nine)(?:.*(?P<last>\d|one|two|three|four|five|six|seven|eight|nine))?.*$").unwrap();
    let result: u32 = re.captures_iter(input).fold(0u32, |acc, c| {
      let first = c.name("first").unwrap();
      let last = c.name("last").unwrap_or(first);

      let first_digit = match first.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        v => v,
      };

      let last_digit = match last.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        v => v,
      };

      let value_string = first_digit.to_string() + last_digit;
      let value = value_string.parse::<u32>().unwrap();

      acc + value
    });

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_PART_ONE: &str = include_str!("../../inputs/samples/day1.txt");
  const SAMPLE_PART_TWO: &str = include_str!("../../inputs/samples/day1_part2.txt");

  #[test]
  fn test_part_one() {
    let command = Day1 {};

    assert_eq!(command.execute_part_one(SAMPLE_PART_ONE), 142);
  }

  #[test]
  fn test_part_two() {
    let command = Day1 {};

    assert_eq!(command.execute_part_two(SAMPLE_PART_ONE), 142);
    assert_eq!(command.execute_part_two(SAMPLE_PART_TWO), 281);
  }
}
