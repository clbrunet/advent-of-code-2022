use std::{
    collections::VecDeque,
    ops::{Add, Mul},
};

#[derive(Debug, Clone)]
enum Term {
    Litteral(u64),
    WorryLevel,
}

impl From<&str> for Term {
    fn from(str: &str) -> Self {
        match str {
            "old" => Self::WorryLevel,
            litteral => Self::Litteral(litteral.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    lhs: Term,
    operation: fn(u64, u64) -> u64,
    rhs: Term,
}

impl From<&str> for Operation {
    fn from(str: &str) -> Self {
        let mut words = str.split_whitespace();
        Self {
            lhs: Term::from(words.next().unwrap()),
            operation: match words.next().unwrap() {
                "+" => u64::add,
                "*" => u64::mul,
                operator => panic!("wrong operator '{}'", operator),
            },
            rhs: Term::from(words.next().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test_divisor: u64,
    true_case_index: usize,
    false_case_index: usize,
    inspection_count: usize,
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        operation: Operation,
        test_divisor: u64,
        true_case_index: usize,
        false_case_index: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test_divisor,
            true_case_index,
            false_case_index,
            inspection_count: 0,
        }
    }
}

impl From<&str> for Monkey {
    fn from(str: &str) -> Self {
        let mut lines = str.lines().skip(1);
        let (_, items) = lines.next().unwrap().split_once(": ").unwrap();
        let items = items
            .split(", ")
            .map(|worry_level| worry_level.parse::<u64>().unwrap())
            .collect();
        let (_, operation) = lines.next().unwrap().split_once("= ").unwrap();
        let operation = Operation::from(operation);
        let test_divisor = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let true_case_index = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_case_index = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Monkey::new(
            items,
            operation,
            test_divisor,
            true_case_index,
            false_case_index,
        )
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| Monkey::from(monkey))
        .collect()
}

fn turn(
    monkey: &mut Monkey,
    true_case_monkey: &mut Monkey,
    false_case_monkey: &mut Monkey,
    should_divide_by_3: bool,
    product: u64,
) {
    monkey.inspection_count += monkey.items.len();
    for mut item in monkey.items.drain(..) {
        let lhs = match monkey.operation.lhs {
            Term::Litteral(litteral) => litteral,
            Term::WorryLevel => item,
        };
        let rhs = match monkey.operation.rhs {
            Term::Litteral(litteral) => litteral,
            Term::WorryLevel => item,
        };
        item = (monkey.operation.operation)(lhs, rhs);
        item %= product;
        if should_divide_by_3 {
            item /= 3;
        }
        if item % monkey.test_divisor == 0 {
            true_case_monkey.items.push_back(item);
        } else {
            false_case_monkey.items.push_back(item);
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>, should_divide_by_3: bool, product: u64) {
    for i in 0..monkeys.len() {
        let mut monkey = monkeys[i].clone();
        let mut true_case_monkey = monkeys[monkey.true_case_index].clone();
        let mut false_case_monkey = monkeys[monkey.false_case_index].clone();
        turn(
            &mut monkey,
            &mut true_case_monkey,
            &mut false_case_monkey,
            should_divide_by_3,
            product,
        );
        monkeys[monkey.true_case_index] = true_case_monkey;
        monkeys[monkey.false_case_index] = false_case_monkey;
        monkeys[i] = monkey;
    }
}

fn simulate(input: &str, round_count: usize, should_divide_by_3: bool) -> String {
    let mut monkeys = parse_input(input);
    let product = monkeys.iter().map(|monkey| monkey.test_divisor).product::<u64>();
    for _ in 0..round_count {
        round(&mut monkeys, should_divide_by_3, product);
    }
    let mut inspection_counts = monkeys
        .into_iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<_>>();
    inspection_counts.sort_unstable();
    let mut inspection_counts = inspection_counts.iter().rev();
    (inspection_counts.next().unwrap() * inspection_counts.next().unwrap()).to_string()
}

pub fn part_1(input: &str) -> String {
    simulate(input, 20, true)
}

pub fn part_2(input: &str) -> String {
    simulate(input, 10_000, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
If true: throw to monkey 2
If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
If true: throw to monkey 2
If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
If true: throw to monkey 1
If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
If true: throw to monkey 0
If false: throw to monkey 1
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "10605");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "2713310158");
    }
}
