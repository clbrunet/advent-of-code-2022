#![feature(iter_array_chunks)]

fn get_priority(ch: char) -> u32 {
    if ch.is_lowercase() {
        ch as u32 - 'a' as u32 + 1
    } else if ch.is_uppercase() {
        ch as u32 - 'A' as u32 + 27
    } else {
        0
    }
}

pub fn part_1(input: &str) {
    let sum = input.lines().fold(0, |accum, line| {
        let (first, second) = line.split_at(line.len() / 2);
        let ch = first.chars().find(|&ch| second.contains(ch)).unwrap();
        accum + get_priority(ch)
    });
    println!("Part 1 : {sum}");
}

pub fn part_2(input: &str) {
    let sum = input.lines().array_chunks().fold(0, |accum, [a, b, c]| {
        let ch = a
            .chars()
            .find(|&ch| b.contains(ch) && c.contains(ch))
            .unwrap();
        accum + get_priority(ch)
    });
    println!("Part 2 : {sum}");
}
