use std::collections::HashMap;
use std::{env, fs};

mod days;
use days::*;

pub type Day = fn(&str) -> u64;

pub fn get_day(year: u32, day: u32) -> (Day, Day) {
    let mut map = HashMap::new();

    let twenty_one: [(Day, Day); 5] = [
        (twenty_one::day_01::part_1, twenty_one::day_01::part_2),
        (twenty_one::day_02::part_1, twenty_one::day_02::part_2),
        (twenty_one::day_03::part_1, twenty_one::day_03::part_2),
        (twenty_one::day_04::part_1, twenty_one::day_04::part_2),
        (twenty_one::day_05::part_1, twenty_one::day_05::part_2),
    ];

    let twenty_two: [(Day, Day); 5] = [
        (twenty_two::day_01::part_1, twenty_two::day_01::part_2),
        (twenty_two::day_01::part_1, twenty_two::day_01::part_2),
        (twenty_two::day_01::part_1, twenty_two::day_01::part_2),
        (twenty_two::day_01::part_1, twenty_two::day_01::part_2),
        (twenty_two::day_01::part_1, twenty_two::day_01::part_2),
    ];

    map.insert(2021, twenty_one);
    map.insert(2022, twenty_two);

    match map.get(&year) {
        Some(&days) => days[(day - 1) as usize],
        _ => unreachable!(),
    }
}

pub fn get_input(year: u32, day: u32, directory: &str) -> String {
    let filename = env::current_dir()
        .unwrap()
        .join(directory)
        .join(format!("{}", year))
        .join(format!("day_{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}
