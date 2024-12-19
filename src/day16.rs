use num::Complex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use pathfinding::prelude::{astar, astar_bag};

type Pos = Complex<isize>;
type Dir = Complex<isize>;
type Grid = HashMap<Pos, Element>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Element {
    Wall,
    Empty,
}

pub fn parse_input(input: &str) -> (Pos, Pos, Grid) {
    let mut grid: Grid = HashMap::default();
    let mut start = Complex::i();
    let mut target = Complex::i();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let pos = Complex::new(i as isize, j as isize);
            let e = match c {
                'S' => {
                    start = pos;
                    Element::Empty
                }
                'E' => {
                    target = pos;
                    Element::Empty
                }
                '.' => Element::Empty,
                '#' => Element::Wall,
                _ => panic!("Unknown character {c}"),
            };
            grid.insert(pos, e);
        }
    }
    (start, target, grid)
}

fn successors(pos: &Pos, dir: &Dir, grid: &Grid) -> Vec<((Pos, Dir), isize)> {
    let mut next = vec![];
    // One cost to keep going in the same direction
    if let Some(e) = grid.get(&(pos + dir)) {
        if *e == Element::Empty {
            next.push(((*pos + *dir, *dir), 1));
        }
    }
    // Cost increased by 1000 to rotate once
    for rotation in [Complex::i(), -Complex::i()] {
        let new_dir = dir * rotation;
        if let Some(e) = grid.get(&(pos + new_dir)) {
            if *e == Element::Empty {
                next.push(((*pos + new_dir, new_dir), 1001));
            }
        }
    }
    next
}

pub fn part1((start, target, grid): (Pos, Pos, Grid)) -> isize {
    let initial_dir = Complex::new(1, 0);
    if let Some((_, cost)) = astar(
        &(start, initial_dir),
        |(pos, dir)| successors(pos, dir, &grid),
        |(pos, _)| (target - pos).l1_norm(),
        |(pos, _)| *pos == target,
    ) {
        cost
    } else {
        panic!("no solution found");
    }
}

pub fn part2((start, target, grid): (Pos, Pos, Grid)) -> usize {
    let initial_dir = Complex::new(1, 0);
    let mut best_pos: HashSet<Pos> = HashSet::default();
    if let Some((solutions, _cost)) = astar_bag(
        &(start, initial_dir),
        |(pos, dir)| successors(pos, dir, &grid),
        |(pos, _)| (target - pos).l1_norm(),
        |(pos, _)| *pos == target,
    ) {
        for path in solutions {
            let positions: HashSet<Pos> = path.iter().map(|(p, _)| *p).collect();
            best_pos.extend(positions);
        }
        best_pos.len()
    } else {
        panic!("No solution found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############"
    };

    #[test]
    fn test_parse_input() {
        let (start, target, grid) = parse_input(INPUT);
        assert_eq!(start, Complex::new(1, 13));
        assert_eq!(target, Complex::new(13, 1));
        assert_eq!(grid[&Complex::new(0, 0)], Element::Wall);
        assert_eq!(grid[&Complex::new(1, 1)], Element::Empty);
        assert_eq!(grid[&Complex::new(1, 13)], Element::Empty);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 7036);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(input), 45);
    }
}
