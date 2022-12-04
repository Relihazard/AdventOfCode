use std::cmp;

pub fn part_1(input: &str) -> u64 {
    let mut max: u64 = 0;
    let mut tmp: u64 = 0;

    for line in input.lines() {
        if line.is_empty() {
            max = cmp::max(tmp, max);
            tmp = 0;
        } else {
            tmp += line.parse::<u64>().unwrap();
        }
    }

    max
}

pub fn part_2(input: &str) -> u64 {
    let mut vec: Vec<u64> = Vec::new();
    let mut tmp: u64 = 0;

    for line in input.lines() {
        if line.is_empty() {
            vec.push(tmp);
            tmp = 0;
        } else {
            tmp += line.parse::<u64>().unwrap();
        }
    }
    if tmp > 0 {
        vec.push(tmp);
    }
    vec.sort();

    vec.iter().rev().take(3).sum()
}
