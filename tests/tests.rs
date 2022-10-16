use advent_of_code::get_day;
use std::{env, fs};

fn get_test_input(day: u32) -> String {
    let filename = env::current_dir()
        .unwrap()
        .join("tests")
        .join("inputs")
        .join(format!("day_{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}

#[cfg(test)]
mod day_01 {
    use super::*;

    const DAY: u32 = 1;

    #[test]
    fn test_part_1() {
        assert_eq!(7, get_day(DAY).0(&get_test_input(DAY)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(5, get_day(DAY).1(&get_test_input(DAY)));
    }
}

#[cfg(test)]
mod day_02 {
    use super::*;

    const DAY: u32 = 2;

    #[test]
    fn test_part_1() {
        assert_eq!(150, get_day(DAY).0(&get_test_input(DAY)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(900, get_day(DAY).1(&get_test_input(DAY)));
    }
}

#[cfg(test)]
mod day_03 {
    use super::*;

    const DAY: u32 = 3;

    #[test]
    fn test_part_1() {
        assert_eq!(198, get_day(DAY).0(&get_test_input(DAY)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(230, get_day(DAY).1(&get_test_input(DAY)));
    }
}

#[cfg(test)]
mod day_04 {
    use super::*;

    const DAY: u32 = 4;

    #[test]
    fn test_part_1() {
        assert_eq!(4512, get_day(DAY).0(&get_test_input(DAY)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1924, get_day(DAY).1(&get_test_input(DAY)));
    }
}
