use std::{env, fs};

pub fn get_test_input(day: u32) -> String {
    let filename = env::current_dir()
        .unwrap()
        .join("tests")
        .join("inputs")
        .join(format!("day_{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}
