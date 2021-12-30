use advent_of_code::get_day;

mod common;

const DAY: u32 = 2;

#[test]
fn test_part_1() {
    assert_eq!(150, get_day(DAY).0(&common::get_test_input(DAY)));
}

#[test]
fn test_part_2() {
    assert_eq!(900, get_day(DAY).1(&common::get_test_input(DAY)));
}
