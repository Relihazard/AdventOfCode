use clap::{App, Arg};

use advent_of_code::{get_day, get_input};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new("Advent of Code")
        .version(VERSION.unwrap_or("unknown"))
        .author(AUTHORS.unwrap_or("Unknown authors"))
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(true)
                .help("The year for which you want the days to be executed"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("The day to be executed"),
        )
        .arg(
            Arg::with_name("directory")
                .short("D")
                .long("directory")
                .takes_value(true)
                .help("The directory containing the input files"),
        )
        .get_matches();

    let year = matches.value_of("year").unwrap_or("2021").parse::<u32>();
    let day = matches.value_of("day").unwrap_or("1").parse::<u32>();
    let directory = matches.value_of("directory").unwrap_or("inputs");

    match (year, day) {
        (Ok(n), Ok(m)) => {
            let parts = get_day(n, m);
            let input = get_input(n, m, directory);
            println!("Year {}, Day {}, part 1: {}", n, m, parts.0(&input));
            println!("Year {}, Day {}, part 2: {}", n, m, parts.1(&input));
        }
        (Err(n), _) => println!("That's not a number! {}", n),
        (_, Err(n)) => println!("That's not a number! {}", n),
    }
}
