pub mod input_fetcher;
pub mod solutions;
pub mod utils;

use input_fetcher::fetch_input;
use solutions::{Solution, D1, D2, D3, D4, D5, D6, D7};

use clap::Args;
use clap::Parser;

#[derive(Parser, Debug)]
struct Options {
    /// The day whose solution will be executed
    #[arg()]
    day: u32,

    /// Use a custom input file
    #[command(flatten)]
    custom_input: Option<CustomInput>,
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
struct CustomInput {
    /// Custom input file
    ///
    /// Prompts the program to use the file at the given input path
    /// rather than the one automatically downloaded from the AOC website
    #[arg(long)]
    input_file: Option<String>,

    /// Custom input string
    ///
    /// Prompts the program to use the provided string as input
    /// rather than the one automatically downloaded from the AOC website
    #[arg(long)]
    input_literal: Option<String>,
}

fn main() {
    let args = Options::parse();

    let day = args.day;

    let input = if let Some(CustomInput {
        input_literal: Some(literal),
        ..
    }) = args.custom_input
    {
        literal
    } else {
        fetch_input(day)
    };

    let mut runner: Box<dyn Solution> = match day {
        1 => Box::new(D1::default()),
        2 => Box::new(D2::default()),
        3 => Box::new(D3::default()),
        4 => Box::new(D4::default()),
        5 => Box::new(D5::default()),
        6 => Box::new(D6::default()),
        7 => Box::new(D7::default()),
        _ => todo!("Not yet implemented"),
    };

    runner.prepare(input);

    println!("P1: {}", runner.p1());
    println!("P2: {}", runner.p2());
}
