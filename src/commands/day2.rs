use clap::Parser;
use regex::Regex;

use super::AoCCommand;

#[derive(Parser)]
pub struct Day2 {}

impl AoCCommand for Day2 {
  fn execute(&self, part_two: bool, sample: bool) {
    let input = include_str!("../../inputs/day2.txt");
    let input_sample = include_str!("../../inputs/samples/day2.txt");

    if part_two {
      self.execute_part_two(if sample { input_sample } else { input });
    } else {
      self.execute_part_one(if sample { input_sample } else { input });
    }
  }

  fn execute_part_one(&self, input: &str) {
    let max_red = 12u32;
    let max_green = 13u32;
    let max_blue = 14u32;

    let game_re = Regex::new(r"(?m)Game (?<id>\d+): (?<cubes>.*)").unwrap();
    let color_value_re: Regex = Regex::new(r"(?<value>\d+) (?<color>red|green|blue)").unwrap();

    let result: u32 = game_re.captures_iter(input).fold(0u32, |acc, c| {
      let id = c.name("id").unwrap().as_str().parse::<u32>().unwrap();
      let cubes = c.name("cubes").unwrap().as_str();

      for subset in cubes.split("; ") {
        for color_value in color_value_re.captures_iter(subset) {
          let color = color_value.name("color").unwrap().as_str();
          let value = color_value
            .name("value")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

          if (color == "red" && value > max_red)
            || (color == "green" && value > max_green)
            || (color == "blue" && value > max_blue)
          {
            return acc;
          }
        }
      }

      acc + id
    });

    println!("{}", result);
  }

  fn execute_part_two(&self, input: &str) {
    let game_re = Regex::new(r"(?m)Game \d+: (?<cubes>.*)").unwrap();
    let color_value_re: Regex = Regex::new(r"(?<value>\d+) (?<color>red|green|blue)").unwrap();

    let result: u32 = game_re.captures_iter(input).fold(0u32, |acc, c| {
      let cubes = c.name("cubes").unwrap().as_str();

      let mut min_red = 0u32;
      let mut min_green = 0u32;
      let mut min_blue = 0u32;

      for subset in cubes.split("; ") {
        for color_value in color_value_re.captures_iter(subset) {
          let color = color_value.name("color").unwrap().as_str();
          let value = color_value
            .name("value")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

          if color == "red" && value > min_red {
            min_red = value;
          }

          if color == "green" && value > min_green {
            min_green = value;
          }

          if color == "blue" && value > min_blue {
            min_blue = value;
          }
        }
      }

      let power = min_red * min_green * min_blue;

      acc + power
    });

    println!("{}", result);
  }
}
