const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn new(char: char) -> Self {
        match char {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!(),
        }
    }

    fn get_shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_outcome_score(&self, opponent: &Self) -> u32 {
        match self {
            Self::Rock => match opponent {
                Self::Rock => DRAW_SCORE,
                Self::Paper => LOSE_SCORE,
                Self::Scissors => WIN_SCORE,
            },
            Self::Paper => match opponent {
                Self::Rock => WIN_SCORE,
                Self::Paper => DRAW_SCORE,
                Self::Scissors => LOSE_SCORE,
            },
            Self::Scissors => match opponent {
                Self::Rock => LOSE_SCORE,
                Self::Paper => WIN_SCORE,
                Self::Scissors => DRAW_SCORE,
            },
        }
    }

    fn get_response(&self, round_result: &RoundResult) -> Self {
        match self {
            Self::Rock => match round_result {
                RoundResult::LOSE => Self::Scissors,
                RoundResult::DRAW => Self::Rock,
                RoundResult::WIN => Self::Paper,
            },
            Self::Paper => match round_result {
                RoundResult::LOSE => Self::Rock,
                RoundResult::DRAW => Self::Paper,
                RoundResult::WIN => Self::Scissors,
            },
            Self::Scissors => match round_result {
                RoundResult::LOSE => Self::Paper,
                RoundResult::DRAW => Self::Scissors,
                RoundResult::WIN => Self::Rock,
            },
        }
    }
}

pub fn part_1(input: &str) {
    let mut score = 0;
    for line in input.lines() {
        let opponent = Shape::new(line.chars().nth(0).unwrap());
        let me = Shape::new(line.chars().nth(2).unwrap());
        score += me.get_shape_score();
        score += me.get_outcome_score(&opponent);
    }
    println!("Part 1 : {}", score);
}

#[derive(Debug)]
enum RoundResult {
    LOSE,
    DRAW,
    WIN,
}
impl RoundResult {
    fn new(char: char) -> Self {
        match char {
            'X' => Self::LOSE,
            'Y' => Self::DRAW,
            'Z' => Self::WIN,
            _ => panic!(),
        }
    }

    fn get_outcome_score(&self) -> u32 {
        match self {
            Self::LOSE => LOSE_SCORE,
            Self::DRAW => DRAW_SCORE,
            Self::WIN => WIN_SCORE,
        }
    }
}

pub fn part_2(input: &str) {
    let mut score = 0;
    for line in input.lines() {
        let opponent = Shape::new(line.chars().nth(0).unwrap());
        let round_result = RoundResult::new(line.chars().nth(2).unwrap());
        score += opponent.get_response(&round_result).get_shape_score();
        score += round_result.get_outcome_score();
    }
    println!("Part 2 : {}", score);
}
