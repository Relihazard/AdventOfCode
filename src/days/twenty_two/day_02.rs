type Score = u64;

enum Choice {
    Rock,
    Paper,
    Scissor,
}

enum RoundResult {
    Win,
    Draw,
    Defeat,
}

impl From<&str> for Choice {
    fn from(choice: &str) -> Self {
        match choice {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissor,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for RoundResult {
    fn from(choice: &str) -> Self {
        match choice {
            "X" => RoundResult::Defeat,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => unreachable!(),
        }
    }
}

impl From<Choice> for Score {
    fn from(choice: Choice) -> Self {
        match choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
}

impl From<RoundResult> for Score {
    fn from(result: RoundResult) -> Self {
        match result {
            RoundResult::Defeat => 0,
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
        }
    }
}

pub fn part_1(input: &str) -> u64 {
    let mut score: u64 = 0;
    for line in input.lines() {
        let calls = line.split(' ').collect::<Vec<&str>>();
        let choice = Choice::from(calls[0]);
        let result = Choice::from(calls[1]);
        match (choice, result) {
            (Choice::Rock, Choice::Rock) => {
                score += Score::from(Choice::Rock) + Score::from(RoundResult::Draw)
            }
            (Choice::Rock, Choice::Paper) => {
                score += Score::from(Choice::Paper) + Score::from(RoundResult::Win)
            }
            (Choice::Rock, Choice::Scissor) => score += Score::from(Choice::Scissor),
            (Choice::Paper, Choice::Rock) => score += Score::from(Choice::Rock),
            (Choice::Paper, Choice::Paper) => {
                score += Score::from(Choice::Paper) + Score::from(RoundResult::Draw)
            }
            (Choice::Paper, Choice::Scissor) => {
                score += Score::from(Choice::Scissor) + Score::from(RoundResult::Win)
            }
            (Choice::Scissor, Choice::Rock) => {
                score += Score::from(Choice::Rock) + Score::from(RoundResult::Win)
            }
            (Choice::Scissor, Choice::Paper) => score += Score::from(Choice::Paper),
            (Choice::Scissor, Choice::Scissor) => {
                score += Score::from(Choice::Scissor) + Score::from(RoundResult::Draw)
            }
        }
    }

    score
}

pub fn part_2(input: &str) -> u64 {
    let mut score: u64 = 0;
    for line in input.lines() {
        let calls = line.split(' ').collect::<Vec<&str>>();
        let choice = Choice::from(calls[0]);
        let result = RoundResult::from(calls[1]);
        match (choice, result) {
            (Choice::Rock, RoundResult::Draw) => {
                score += Score::from(Choice::Rock) + Score::from(RoundResult::Draw)
            }
            (Choice::Rock, RoundResult::Win) => {
                score += Score::from(Choice::Paper) + Score::from(RoundResult::Win)
            }
            (Choice::Rock, RoundResult::Defeat) => score += Score::from(Choice::Scissor),
            (Choice::Paper, RoundResult::Draw) => {
                score += Score::from(Choice::Paper) + Score::from(RoundResult::Draw)
            }
            (Choice::Paper, RoundResult::Defeat) => score += Score::from(Choice::Rock),
            (Choice::Paper, RoundResult::Win) => {
                score += Score::from(Choice::Scissor) + Score::from(RoundResult::Win)
            }
            (Choice::Scissor, RoundResult::Draw) => {
                score += Score::from(Choice::Scissor) + Score::from(RoundResult::Draw)
            }
            (Choice::Scissor, RoundResult::Defeat) => score += Score::from(Choice::Paper),
            (Choice::Scissor, RoundResult::Win) => {
                score += Score::from(Choice::Rock) + Score::from(RoundResult::Win)
            }
        }
    }

    score
}
