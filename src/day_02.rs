pub fn part_1(input: &str) -> u64 {
    let mut positions = (0, 0);
    for x in parse_input(input) {
        match x.0 {
            "forward" => positions.0 += x.1,
            "up" => positions.1 -= x.1,
            "down" => positions.1 += x.1,
            _ => panic!("Unknow direction: {}", x.0),
        }
    }
    return positions.0 * positions.1;
}

pub fn part_2(input: &str) -> u64 {
    let mut positions = (0, 0);
    let mut aim = 0;
    for x in parse_input(input) {
        match x.0 {
            "forward" => {
                positions.0 += x.1;
                positions.1 += aim * x.1
            }
            "up" => aim -= x.1,
            "down" => aim += x.1,
            _ => panic!("Unknow direction: {}", x.0),
        }
    }
    return positions.0 * positions.1;
}

fn parse_input(input: &str) -> Vec<(&str, u64)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            (split[0], split[1].parse::<u64>().unwrap())
        })
        .collect()
}
