fn get_stacks(input_stacks: &str) -> Vec<Vec<char>> {
    let mut input_stacks = input_stacks.lines().rev().collect::<Vec<&str>>();
    let stacks_count = input_stacks
        .remove(0)
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stacks_count];
    let input_stacks = input_stacks
        .iter()
        .map(|&line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for (i, stack) in stacks.iter_mut().enumerate() {
        for line in input_stacks.iter() {
            let ch = line[i * 4 + 1];
            if !ch.is_alphabetic() {
                break;
            }
            stack.push(ch);
        }
    }
    stacks
}
struct Rearrangement {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Rearrangement {
    fn new(line: &str) -> Self {
        const QUANTITY_WORD_INDEX: usize = 1;
        const FROM_WORD_INDEX: usize = 3;
        const TO_WORD_INDEX: usize = 5;

        let words = line.split_whitespace().collect::<Vec<&str>>();
        Self {
            quantity: words[QUANTITY_WORD_INDEX].parse::<usize>().unwrap(),
            from: words[FROM_WORD_INDEX].parse::<usize>().unwrap() - 1,
            to: words[TO_WORD_INDEX].parse::<usize>().unwrap() - 1,
        }
    }
}

pub fn part_1(input: &str) -> String {
    let (input_stacks, rearrangements) = input.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(input_stacks);
    for line in rearrangements.lines() {
        let rearrangement = Rearrangement::new(line);
        for _ in 0..rearrangement.quantity {
            let ch = stacks[rearrangement.from].pop().unwrap();
            stacks[rearrangement.to].push(ch);
        }
    }
    let result = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
    result
}

pub fn part_2(input: &str) -> String {
    let (input_stacks, rearrangements) = input.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(input_stacks);
    for line in rearrangements.lines() {
        let rearrangement = Rearrangement::new(line);
        let len = stacks[rearrangement.from].len();
        let mut off = stacks[rearrangement.from].split_off(len - rearrangement.quantity);
        stacks[rearrangement.to].append(&mut off);
    }
    let result = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "CMZ");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "MCD");
    }
}
