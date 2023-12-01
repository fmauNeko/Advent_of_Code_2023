use clap::Parser;

use crate::commands::Subcommands;

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[arg(short, long)]
  sample: bool,

  #[arg(short = '2', long)]
  part_two: bool,

  #[command(subcommand)]
  command: Option<Subcommands>,
}

fn main() {
  let cli = Cli::parse();
  let command = cli.command.unwrap();

  command.execute(cli.part_two, cli.sample);
}
