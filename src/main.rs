use clap::{App, Arg};

use advent_of_code::{get_day, get_input};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version(VERSION.unwrap_or("unknown"))
        .author(AUTHORS.unwrap_or("Unknown authors"))
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("The day to be executed"),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap_or("1").parse::<u32>();
    match day {
        Ok(n) => {
            let parts = get_day(n);
            let input = get_input(n);
            println!("Day {}, part 1: {}", n, parts.0(&input));
            println!("Day {}, part 2: {}", n, parts.1(&input));
        }
        Err(n) => println!("That's not a number! {}", n),
    }
}
