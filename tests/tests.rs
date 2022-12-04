use advent_of_code::get_day;
use std::{env, fs};

const TEST_DIRECTORY: &str = "tests/inputs";

fn get_test_input(year: u32, day: u32) -> String {
    let filename = env::current_dir()
        .unwrap()
        .join(TEST_DIRECTORY)
        .join(format!("{}", year))
        .join(format!("day_{:02}", day));
    fs::read_to_string(filename).expect("Error while reading")
}

#[cfg(test)]
mod twenty_one {
    use super::{get_day, get_test_input};

    const YEAR: u32 = 2021;

    #[cfg(test)]
    mod day_01 {
        use super::*;

        const DAY: u32 = 1;

        #[test]
        fn test_part_1() {
            assert_eq!(7, get_day(YEAR, DAY).0(&get_test_input(YEAR, DAY)));
        }

        #[test]
        fn test_part_2() {
            assert_eq!(5, get_day(YEAR, DAY).1(&get_test_input(YEAR, DAY)));
        }
    }

    #[cfg(test)]
    mod day_02 {
        use super::*;

        const DAY: u32 = 2;

        #[test]
        fn test_part_1() {
            assert_eq!(150, get_day(YEAR, DAY).0(&get_test_input(YEAR, DAY)));
        }

        #[test]
        fn test_part_2() {
            assert_eq!(900, get_day(YEAR, DAY).1(&get_test_input(YEAR, DAY)));
        }
    }

    #[cfg(test)]
    mod day_03 {
        use super::*;

        const DAY: u32 = 3;

        #[test]
        fn test_part_1() {
            assert_eq!(198, get_day(YEAR, DAY).0(&get_test_input(YEAR, DAY)));
        }

        #[test]
        fn test_part_2() {
            assert_eq!(230, get_day(YEAR, DAY).1(&get_test_input(YEAR, DAY)));
        }
    }

    #[cfg(test)]
    mod day_04 {
        use super::*;

        const DAY: u32 = 4;

        #[test]
        fn test_part_1() {
            assert_eq!(4512, get_day(YEAR, DAY).0(&get_test_input(YEAR, DAY)));
        }

        #[test]
        fn test_part_2() {
            assert_eq!(1924, get_day(YEAR, DAY).1(&get_test_input(YEAR, DAY)));
        }
    }
}

#[cfg(test)]
mod twenty_two {
    use super::{get_day, get_test_input};

    const YEAR: u32 = 2022;

    #[cfg(test)]
    mod day_01 {
        use super::*;

        const DAY: u32 = 1;

        #[test]
        fn test_part_1() {
            assert_eq!(24000, get_day(YEAR, DAY).0(&get_test_input(YEAR, DAY)));
        }

        #[test]
        fn test_part_2() {
            assert_eq!(45000, get_day(YEAR, DAY).1(&get_test_input(YEAR, DAY)));
        }
    }
}
