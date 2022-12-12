#[derive(Debug, Default, PartialEq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    elevations: Vec<Vec<u32>>,
    start: Coord,
    end: Coord,
}

fn get_elevation(char: char) -> u32 {
    char as u32 - 'a' as u32
}

impl From<&str> for Grid {
    fn from(str: &str) -> Self {
        let mut start = Coord::default();
        let mut end = Coord::default();
        let elevations = str
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, char)| match char {
                        'S' => {
                            start = Coord::new(j, i);
                            get_elevation('a')
                        }
                        'E' => {
                            end = Coord::new(j, i);
                            get_elevation('z')
                        }
                        letter if letter.is_lowercase() => get_elevation(letter),
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            elevations,
            start,
            end,
        }
    }
}

fn is_next_coord_valid(
    next_coord: &Coord,
    elevations: &[Vec<u32>],
    max_next_elevation: u32,
    lowest_steps_grid: &[Vec<usize>],
    coords: &[Coord],
) -> bool {
    elevations[next_coord.y][next_coord.x] <= max_next_elevation
        && lowest_steps_grid[next_coord.y][next_coord.x] > coords.len()
        && coords.iter().all(|coord| *coord != *next_coord)
}

fn add_next_coord(
    next_coord: Coord,
    lowest_steps_grid: &mut [Vec<usize>],
    coords: &mut Vec<Coord>,
    taken_directions: &mut Vec<u8>,
) {
    lowest_steps_grid[next_coord.y][next_coord.x] = coords.len();
    coords.push(next_coord);
    taken_directions.push(0);
}

fn step(
    grid: &Grid,
    steps: &mut Vec<usize>,
    coords: &mut Vec<Coord>,
    taken_directions: &mut Vec<u8>,
    lowest_steps_grid: &mut [Vec<usize>],
) {
    let coord = coords.last().unwrap();
    if *coord == grid.end {
        coords.pop();
        taken_directions.pop();
        steps.push(coords.len());
        return;
    }
    let taken_direction = taken_directions.last_mut().unwrap();
    let max_next_elevation = grid.elevations[coord.y][coord.x] + 1;
    if *taken_direction < 1 {
        *taken_direction += 1;
        if coord.y > 0 {
            let next_coord = Coord::new(coord.x, coord.y - 1);
            if is_next_coord_valid(
                &next_coord,
                &grid.elevations,
                max_next_elevation,
                lowest_steps_grid,
                coords,
            ) {
                add_next_coord(next_coord, lowest_steps_grid, coords, taken_directions);
                return;
            }
        }
    }
    if *taken_direction < 2 {
        *taken_direction += 1;
        if coord.x + 1 < grid.elevations[coord.y].len() {
            let next_coord = Coord::new(coord.x + 1, coord.y);
            if is_next_coord_valid(
                &next_coord,
                &grid.elevations,
                max_next_elevation,
                lowest_steps_grid,
                coords,
            ) {
                add_next_coord(next_coord, lowest_steps_grid, coords, taken_directions);
                return;
            }
        }
    }
    if *taken_direction < 3 {
        *taken_direction += 1;
        if coord.y + 1 < grid.elevations.len() {
            let next_coord = Coord::new(coord.x, coord.y + 1);
            if is_next_coord_valid(
                &next_coord,
                &grid.elevations,
                max_next_elevation,
                lowest_steps_grid,
                coords,
            ) {
                add_next_coord(next_coord, lowest_steps_grid, coords, taken_directions);
                return;
            }
        }
    }
    if *taken_direction < 4 {
        *taken_direction += 1;
        if coord.x > 0 {
            let next_coord = Coord::new(coord.x - 1, coord.y);
            if is_next_coord_valid(
                &next_coord,
                &grid.elevations,
                max_next_elevation,
                lowest_steps_grid,
                coords,
            ) {
                add_next_coord(next_coord, lowest_steps_grid, coords, taken_directions);
                return;
            }
        }
    }
    coords.pop();
    taken_directions.pop();
}

// refactor idea: add to each cell of lowest_steps_grid the number of steps to go to the end from
// that cell then if lowest_steps_grid[next_coord.y][next_coord.x] > coords.len() then just decrease
// the number of steps to go to the end
fn path(grid: &Grid) -> Option<usize> {
    let mut steps = Vec::new();
    let mut coords = vec![grid.start.clone()];
    let mut taken_directions = vec![0];
    let mut lowest_steps_grid =
        vec![vec![usize::MAX; grid.elevations[0].len()]; grid.elevations.len()];
    while !coords.is_empty() {
        step(
            grid,
            &mut steps,
            &mut coords,
            &mut taken_directions,
            &mut lowest_steps_grid,
        );
    }
    steps.iter().min().copied()
}

pub fn part_1(input: &str) -> String {
    let grid = Grid::from(input);
    path(&grid).unwrap().to_string()
}

pub fn part_2(input: &str) -> String {
    let mut grid = Grid::from(input);
    let starts = grid
        .elevations
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, elevation)| (*elevation == 0).then(|| Coord::new(x, y)))
        })
        .collect::<Vec<_>>();
    starts
        .into_iter()
        .filter_map(|start| {
            grid.start = start;
            path(&grid)
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "31");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "29");
    }
}
