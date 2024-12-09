use num::Complex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Dir = Complex<isize>;
type Grid = HashMap<Pos, State>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Empty,
    Obstacle,
}

pub fn parse_input(input: &str) -> (Pos, Grid) {
    let mut start = Complex::default();
    let mut grid = HashMap::default();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let (pos, state) = match c {
                '^' => {
                    start = Complex::new(i as isize, j as isize);
                    (Complex::new(i as isize, j as isize), State::Empty)
                }
                '.' => (Complex::new(i as isize, j as isize), State::Empty),
                '#' => (Complex::new(i as isize, j as isize), State::Obstacle),
                _ => panic!("unexpected character"),
            };
            grid.insert(pos, state);
        }
    }
    (start, grid)
}

pub fn part1((start, grid): (Pos, Grid)) -> usize {
    // we are facing up, but the imaginary axis is backwards
    let mut direction = -Complex::i();
    let mut pos = start;
    let mut steps = HashSet::default();
    steps.insert(pos);
    while let Some(state) = grid.get(&(pos + direction)) {
        match state {
            State::Empty => {
                pos += direction;
                steps.insert(pos);
            }
            State::Obstacle => direction *= Complex::i(),
        }
    }
    steps.len()
}

fn _print_grid_and_steps(grid: &Grid, steps: &HashSet<(Pos, Dir)>) {
    let size = grid.iter().map(|(a, _)| a.re).max().unwrap();
    for j in 0..=size {
        let mut s = String::new();
        for i in 0..=size {
            let pos = Complex::new(i, j);
            let c = match steps.iter().find(|(p, _d)| *p == pos) {
                Some(_) => 'X',
                None => match grid.get(&pos) {
                    Some(state) => match state {
                        State::Empty => '.',
                        State::Obstacle => '#',
                    },
                    None => ' ',
                },
            };
            s = format!("{}{}", s, c);
        }
        println!("{}", s);
    }
    println!();
}

fn find_loop(position: Pos, direction: Dir, steps: &HashSet<(Pos, Dir)>, grid: &Grid) -> bool {
    let mut steps = steps.to_owned();
    // we add a block at position + direction, so the new direction to check is rotated
    // by 90°
    let mut grid = grid.clone();
    grid.insert(position + direction, State::Obstacle);
    let mut direction = direction * Complex::i();
    let mut position = position;
    while let Some(state) = grid.get(&(position + direction)) {
        match state {
            State::Obstacle => {
                direction *= Complex::i();
            }
            State::Empty => {
                // did we reach this point with this direction already?
                if steps.contains(&(position, direction)) {
                    return true;
                }
                steps.insert((position, direction));
                position += direction;
            }
        }
    }
    // _print_grid_and_steps(&grid, &steps;
    false
}

pub fn part2((start, grid): (Pos, Grid)) -> usize {
    let mut direction = -Complex::i();
    let mut pos = start;
    let mut steps = HashSet::default();
    let mut blocks: HashSet<Complex<isize>> = HashSet::default();
    steps.insert((pos, direction));
    while let Some(state) = grid.get(&(pos + direction)) {
        match state {
            State::Empty => {
                steps.insert((pos, direction));
                // we cannot insert a block to a position where the guard has alreaady
                // been or at the start.
                if pos + direction == start || steps.iter().any(|(p, _)| *p == pos + direction) {
                    pos += direction;
                    continue;
                }
                if find_loop(pos, direction, &steps, &grid) {
                    blocks.insert(pos + direction);
                }
                pos += direction;
            }
            State::Obstacle => direction *= Complex::i(),
        }
    }
    blocks.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
        "
    };

    #[test]
    fn test_parse_input() {
        let (start, grid) = parse_input(INPUT);
        assert_eq!(start, Complex::new(4, 6));
        assert_eq!(grid[&Complex::new(4, 6)], State::Empty);
        assert_eq!(grid[&Complex::new(4, 0)], State::Obstacle);
        assert_eq!(grid[&Complex::new(0, 0)], State::Empty);
    }

    #[test]
    fn test_part1() {
        let (start, grid) = parse_input(INPUT);
        assert_eq!(part1((start, grid)), 41);
    }

    #[test]
    fn test_part2() {
        let (start, grid) = parse_input(INPUT);
        assert_eq!(part2((start, grid)), 6);
    }
}
