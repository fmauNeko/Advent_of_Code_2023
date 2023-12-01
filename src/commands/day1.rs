use clap::Parser;
use regex::Regex;

use super::AoCCommand;

#[derive(Parser)]
pub struct Day1 {}

impl AoCCommand for Day1 {
  fn execute(&self, part_two: bool, sample: bool) {
    let input = include_str!("../../inputs/day1.txt");
    let input_part_one_sample = include_str!("../../inputs/samples/day1.txt");
    let input_part_two_sample = include_str!("../../inputs/samples/day1_part2.txt");

    if part_two {
      self.execute_part_two(if sample { input_part_two_sample } else { input });
    } else {
      self.execute_part_one(if sample { input_part_one_sample } else { input });
    }
  }

  fn execute_part_one(&self, input: &str) {
    let re = Regex::new(r"(?m)^\D*(?P<first>\d)(?:.*(?P<last>\d))?\D*$").unwrap();
    let result: u32 = re.captures_iter(input).fold(0u32, |acc, c| {
      let first = c.name("first").unwrap();
      let last = c.name("last").unwrap_or(first);

      let value_string = first.as_str().to_string() + last.as_str();
      let value = value_string.parse::<u32>().unwrap();

      acc + value
    });

    println!("{:?}", result);
  }

  fn execute_part_two(&self, input: &str) {
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

    println!("{:?}", result);
  }
}
