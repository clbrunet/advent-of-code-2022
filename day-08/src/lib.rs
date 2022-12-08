fn get_2d_vec_trees(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_tree_left_visible(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> bool {
    !(0..j).any(|j| trees[i][j] >= height)
}

fn is_tree_right_visible(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> bool {
    !((j + 1)..trees[i].len()).any(|j| trees[i][j] >= height)
}

fn is_tree_up_visible(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> bool {
    !(0..i).any(|i| trees[i][j] >= height)
}

fn is_tree_down_visible(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> bool {
    !((i + 1)..trees.len()).any(|i| trees[i][j] >= height)
}

fn is_tree_visible(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> bool {
    is_tree_left_visible(trees, i, j, height)
        || is_tree_right_visible(trees, i, j, height)
        || is_tree_up_visible(trees, i, j, height)
        || is_tree_down_visible(trees, i, j, height)
}

pub fn part_1(input: &str) -> String {
    let trees = get_2d_vec_trees(input);
    let visible_count =
        trees
            .iter()
            .enumerate()
            .fold(trees.len() * trees[0].len(), |acc, (i, row)| {
                row.iter().enumerate().fold(acc, |acc, (j, &height)| {
                    if !is_tree_visible(&trees, i, j, height) {
                        acc - 1
                    } else {
                        acc
                    }
                })
            });
    visible_count.to_string()
}

fn get_left_scenic_score(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> u32 {
    match (0..j).rev().position(|j| trees[i][j] >= height) {
        Some(index) => index as u32 + 1,
        None => j as u32,
    }
}

fn get_right_scenic_score(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> u32 {
    match ((j + 1)..trees[i].len()).position(|j| trees[i][j] >= height) {
        Some(index) => index as u32 + 1,
        None => (trees[i].len() - (j + 1)) as u32,
    }
}

fn get_up_scenic_score(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> u32 {
    match (0..i).rev().position(|i| trees[i][j] >= height) {
        Some(index) => index as u32 + 1,
        None => i as u32,
    }
}

fn get_down_scenic_score(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> u32 {
    match ((i + 1)..trees.len()).position(|i| trees[i][j] >= height) {
        Some(index) => index as u32 + 1,
        None => (trees.len() - (i + 1)) as u32,
    }
}

fn get_scenic_score(trees: &[Vec<u32>], i: usize, j: usize, height: u32) -> u32 {
    get_left_scenic_score(trees, i, j, height)
        * get_right_scenic_score(trees, i, j, height)
        * get_up_scenic_score(trees, i, j, height)
        * get_down_scenic_score(trees, i, j, height)
}

pub fn part_2(input: &str) -> String {
    let trees = get_2d_vec_trees(input);
    let max = trees
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, height)| get_scenic_score(&trees, i, j, *height))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "21");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "8");
    }
}
