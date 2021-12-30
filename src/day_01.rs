pub fn part_1(input: &str) -> u64 {
    parse_input(input).windows(2).fold(0, |total, window| {
        if window[1] > window[0] {
            total + 1
        } else {
            total
        }
    })
}

pub fn part_2(input: &str) -> u64 {
    let parsed_input = parse_input(input);
    let triplets: Vec<&[u64]> = parsed_input.windows(3).collect();
    triplets.windows(2).fold(0, |total, window| {
        let prev_sum: u64 = window[0].iter().sum();
        let curr_sum: u64 = window[1].iter().sum();
        if curr_sum > prev_sum {
            total + 1
        } else {
            total
        }
    })
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}
