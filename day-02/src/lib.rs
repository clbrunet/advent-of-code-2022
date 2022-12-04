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
                RoundResult::Lose => Self::Scissors,
                RoundResult::Draw => Self::Rock,
                RoundResult::Win => Self::Paper,
            },
            Self::Paper => match round_result {
                RoundResult::Lose => Self::Rock,
                RoundResult::Draw => Self::Paper,
                RoundResult::Win => Self::Scissors,
            },
            Self::Scissors => match round_result {
                RoundResult::Lose => Self::Paper,
                RoundResult::Draw => Self::Scissors,
                RoundResult::Win => Self::Rock,
            },
        }
    }
}

pub fn part_1(input: &str) -> String {
    let score = input.lines().fold(0, |accum, line| {
        let opponent = Shape::new(line.chars().nth(0).unwrap());
        let me = Shape::new(line.chars().nth(2).unwrap());
        accum + me.get_shape_score() + me.get_outcome_score(&opponent)
    });
    score.to_string()
}

#[derive(Debug)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}
impl RoundResult {
    fn new(char: char) -> Self {
        match char {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(),
        }
    }

    fn get_outcome_score(&self) -> u32 {
        match self {
            Self::Lose => LOSE_SCORE,
            Self::Draw => DRAW_SCORE,
            Self::Win => WIN_SCORE,
        }
    }
}

pub fn part_2(input: &str) -> String {
    let score = input.lines().fold(0, |accum, line| {
        let opponent = Shape::new(line.chars().nth(0).unwrap());
        let round_result = RoundResult::new(line.chars().nth(2).unwrap());
        accum
            + opponent.get_response(&round_result).get_shape_score()
            + round_result.get_outcome_score()
    });
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "15");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "12");
    }
}
