use derivative::Derivative;

use num::Complex;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashSet as HashSet;

type Position = Complex<isize>;
type Grid = HashSet<Position>;

pub fn parse_input(input: &str) -> (Grid, Position, Position) {
    let mut start = Complex::default();
    let mut end = Complex::default();
    let mut grid = Grid::default();
    for (j, l) in input.lines().enumerate() {
        for (i, c) in l.chars().enumerate() {
            let pos = Complex::new(i as isize, j as isize);
            match c {
                '#' => (),
                '.' => {
                    grid.insert(pos);
                }
                'S' => {
                    start = pos;
                    grid.insert(pos);
                }
                'E' => {
                    end = pos;
                    grid.insert(pos);
                }
                c => panic!("Unexpected character {c}"),
            }
        }
    }
    (grid, start, end)
}

const DIRECTIONS: [Position; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

#[derive(Derivative, Clone, Debug)]
#[derivative(Hash, PartialEq)]
struct Pos {
    pos: Complex<isize>,
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    can_cheat: bool,
}

impl Eq for Pos {}

impl Pos {
    fn new(pos: Complex<isize>, can_cheat: bool) -> Self {
        Self { pos, can_cheat }
    }

    fn successors(&self, grid: &Grid) -> Vec<(Pos, usize)> {
        let mut next = Vec::new();
        for d in DIRECTIONS {
            if grid.contains(&(self.pos + d)) {
                next.push((Pos::new(self.pos + d, self.can_cheat), 1))
            } else if self.can_cheat && grid.contains(&(self.pos + 2 * d)) {
                next.push((Pos::new(self.pos + 2 * d, false), 2));
            }
        }
        next
    }
}

fn _print_grid(x_max: isize, y_max: isize, grid: &HashSet<Position>, path: &Option<Vec<Position>>) {
    for j in 0..=y_max {
        for i in 0..=x_max {
            let pos = Complex::new(i, j);
            match path {
                None => {
                    if grid.contains(&pos) {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
                Some(p) => {
                    if p.contains(&pos) {
                        print!("O");
                    } else if grid.contains(&pos) {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }
        }
        println!();
    }
    println!();
}

pub fn part1((grid, start, end): (Grid, Position, Position)) -> usize {
    let start = Pos::new(start, false);
    let (base_path, base_score) =
        dijkstra(&start, |p| p.successors(&grid), |p| p.pos == end).unwrap();

    let mut path = Vec::from_iter([(Pos::new(start.pos, true), 0, HashSet::default())]);
    let mut lengths_to_goal = Vec::new();
    while let Some((pos, length, mut seen)) = path.pop() {
        if seen.contains(&pos.pos) {
            continue;
        }
        if length + 100 > base_score {
            continue;
        }
        if !pos.can_cheat {
            // we are back to the base case
            let idx = base_path.iter().position(|p| *p == pos).unwrap();
            // total length will be length + base_score - idx, we compare this to
            // base_score - 100, this gives the inequality below
            if length + 100 <= idx {
                lengths_to_goal.push(length + base_score - idx);
            }
            continue;
        }
        seen.insert(pos.pos);
        if pos.pos == end {
            lengths_to_goal.push(length);
            continue;
        }
        let next_paths = pos.successors(&grid);
        path.extend(
            next_paths
                .into_iter()
                .map(|(p, l)| (p, length + l, seen.clone())),
        );
    }

    lengths_to_goal.len()
    // 1365 ok
}

pub fn part1_2((grid, start, end): (Grid, Position, Position)) -> usize {
    let start = Pos::new(start, false);
    let (base_path, _) = dijkstra(&start, |p| p.successors(&grid), |p| p.pos == end).unwrap();
    find_cheats(&base_path, 2, 101)
}

fn find_cheats(path: &[Pos], can_skip_length: usize, target_skip: usize) -> usize {
    let mut tot = 0;
    for (idx, pos) in path[..path.len() - target_skip].iter().enumerate() {
        tot += path[idx + target_skip..]
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let distance = (pos.pos - p.pos).l1_norm() as usize;
                distance > 1 && distance <= can_skip_length
            })
            .count();
    }

    tot
}

pub fn part2((grid, start, end): (Grid, Position, Position)) -> usize {
    let start = Pos::new(start, false);
    let (base_path, _) = dijkstra(&start, |p| p.successors(&grid), |p| p.pos == end).unwrap();
    find_cheats(&base_path, 20, 101)
    // 1036312 too high
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use num::Zero;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############"
    };

    #[test]
    fn test_parse_input() {
        let (grid, start, end) = parse_input(INPUT);
        assert_eq!(start, Complex::new(1, 3));
        assert_eq!(end, Complex::new(5, 7));
        assert_eq!(grid.get(&Complex::zero()), None);
        assert_eq!(grid.get(&Complex::new(1, 1)), Some(&Complex::new(1, 1)));
        assert_eq!(grid.get(&start), Some(&start));
        assert_eq!(grid.get(&end), Some(&end));
    }

    #[test]
    fn test_find_cheats() {
        let (grid, start, end) = parse_input(INPUT);
        let start = Pos::new(start, false);
        let (base_path, _) = dijkstra(&start, |p| p.successors(&grid), |p| p.pos == end).unwrap();
        // let res = find_cheats(&base_path, 2, 3);
        // assert_eq!(res, 44);
        let res_p2 = find_cheats(&base_path, 20, 77);
        assert_eq!(res_p2, 285);
    }
}
