use std::{env, fs};

mod day_01;

pub type Day = fn(&str) -> u64;

pub fn get_day(day: u32) -> (Day, Day) {
    match day {
        1 => (day_01::part_1, day_01::part_2),
        _ => panic!("Unknown day: {}", day),
    }
}

pub fn get_input(day: u32) -> String {
    let filename = env::current_dir()
        .unwrap()
        .join("inputs")
        .join(format!("day_{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}
