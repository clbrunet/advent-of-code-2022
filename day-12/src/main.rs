use day_12::{part_1, part_2};

fn main() {
    let input = String::from(include_str!("../input.txt"));
    println!("Part 1 : {}", part_1(&input));
    println!("Part 2 : {}", part_2(&input));
}
