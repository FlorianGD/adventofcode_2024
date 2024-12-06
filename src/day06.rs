use num::Complex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Grid = HashMap<Pos, State>;

#[derive(Debug, PartialEq, Eq)]
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
    // we are facing up, but the imaginaty axis is backwards
    let mut direction = Complex::new(0, -1);
    let mut pos = start;
    let mut steps = HashSet::default();
    steps.insert(pos);
    while let Some(state) = grid.get(&(pos + direction)) {
        match state {
            State::Empty => {
                steps.insert(pos);
                pos += direction;
            }
            State::Obstacle => direction *= Complex::new(0, 1),
        }
    }
    steps.len() + 1
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
}
