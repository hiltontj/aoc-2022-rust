use crate::utils::get_lines;

pub fn answer_part_1() -> usize {
    let mut points = 0;
    for line in get_lines("input/day_02.txt") {
        let line = line.expect("reads a line");
        let theirs = line.chars().nth(0).map(Play::try_from).unwrap().unwrap();
        let mine = line.chars().nth(2).map(Play::try_from).unwrap().unwrap();

        points += mine.score(&theirs) + mine.play_points();
    }
    points
}

pub fn answer_part_2() -> usize {
    let mut points = 0;
    for line in get_lines("input/day_02.txt") {
        let line = line.expect("reads a line");
        let theirs = line.chars().nth(0).map(Play::try_from).unwrap().unwrap();
        let desired = line.chars().nth(2).map(Outcome::try_from).unwrap().unwrap();
        let mine = Play::needed_for(&desired, &theirs);
        points += mine.score(&theirs) + mine.play_points();
    }

    points
}

#[derive(PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Play {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(format!("invalid char: {c}")),
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissors, Play::Scissors) => Some(std::cmp::Ordering::Equal),
            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissors)
            | (Play::Scissors, Play::Rock) => Some(std::cmp::Ordering::Less),
            (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl Play {
    fn score(&self, theirs: &Play) -> usize {
        if self > theirs {
            6
        } else if self < theirs {
            0
        } else {
            3
        }
    }

    fn play_points(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn needed_for(desired: &Outcome, theirs: &Play) -> Self {
        match (desired, theirs) {
            (Outcome::Win, Play::Rock) => Self::Paper,
            (Outcome::Win, Play::Paper) => Self::Scissors,
            (Outcome::Win, Play::Scissors) => Self::Rock,
            (Outcome::Draw, Play::Rock) => Self::Rock,
            (Outcome::Draw, Play::Paper) => Self::Paper,
            (Outcome::Draw, Play::Scissors) => Self::Scissors,
            (Outcome::Loss, Play::Rock) => Self::Scissors,
            (Outcome::Loss, Play::Paper) => Self::Rock,
            (Outcome::Loss, Play::Scissors) => Self::Paper,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(format!("invalid char for outcome: {c}")),
        }
    }
}
