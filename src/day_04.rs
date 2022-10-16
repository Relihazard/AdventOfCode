use std::collections::HashMap;

const BOARD_SIZE: usize = 5;

struct Board {
    num_map: HashMap<u64, (usize, usize)>,
    row_count: [usize; BOARD_SIZE],
    col_count: [usize; BOARD_SIZE],
    winning_value: u64,
}

impl Board {
    fn from(nums: Vec<u64>) -> Board {
        let mut num_map = HashMap::<u64, (usize, usize)>::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let i = r * BOARD_SIZE + c;
                num_map.insert(nums[i], (r, c));
            }
        }
        Board {
            num_map,
            row_count: [0; BOARD_SIZE],
            col_count: [0; BOARD_SIZE],
            winning_value: 0,
        }
    }

    fn mark(&mut self, num: u64) -> bool {
        if let Some((r, c)) = self.num_map.remove(&num) {
            self.row_count[r] += 1;
            self.col_count[c] += 1;
            if (self.row_count[r] == BOARD_SIZE || self.col_count[c] == BOARD_SIZE)
                && self.winning_value == 0
            {
                self.winning_value = num;
                return true;
            }
        }
        false
    }

    fn score(&self) -> u64 {
        println!("{}", self.num_map.keys().sum::<u64>());
        self.num_map.keys().sum::<u64>() * self.winning_value
    }
}

pub fn part_1(input: &str) -> u64 {
    let (numbers, mut boards) = parse_input(input);
    let mut iter = boards.iter_mut();
    for n in numbers {
        for b in iter {
            if b.mark(n) {
                return b.score();
            }
        }
        iter = boards.iter_mut();
    }
    0
}

pub fn part_2(input: &str) -> u64 {
    let (numbers, mut boards) = parse_input(input);
    let mut iter = boards.iter_mut();
    let mut len = iter.len();
    let mut last = Vec::new();
    for n in numbers {
        for (i, b) in iter.enumerate() {
            if b.mark(n) {
                if len > 1 && !last.contains(&i) {
                    last.push(i);
                } else if len == 1 {
                    return b.score();
                }
            }
        }
        if !last.is_empty() {
            last.reverse();
            for i in last.to_vec() {
                boards.remove(i);
            }
        }
        iter = boards.iter_mut();
        len = iter.len();
        last.clear();
    }
    0
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Board>) {
    let mut lines = input.lines();
    let numbers: Vec<u64> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let vec = &mut Vec::new();
    let mut boards = Vec::new();

    for line in lines {
        if !line.is_empty() {
            for s in line.split(" ").filter(|s| !s.is_empty()) {
                vec.push(s.trim().parse::<u64>().unwrap())
            }
        } else if !vec.is_empty() {
            boards.push(Board::from(vec.to_vec()));
            vec.clear();
        }
    }
    boards.push(Board::from(vec.to_vec()));
    (numbers, boards)
}
