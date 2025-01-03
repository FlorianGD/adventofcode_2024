use num::Complex;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Grid = HashSet<Pos>;

pub fn parse_input(input: &str) -> (Grid, Pos, Pos) {
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

const DIRECTIONS: [Pos; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

fn successors(pos: &Pos, grid: &Grid) -> Vec<(Pos, usize)> {
    let mut next = Vec::new();
    for d in DIRECTIONS {
        if grid.contains(&(pos + d)) {
            next.push((pos + d, 1))
        }
    }
    next
}

pub fn part1((grid, start, end): (Grid, Pos, Pos)) -> usize {
    let (base_path, _) = dijkstra(&start, |p| successors(p, &grid), |p| *p == end).unwrap();
    find_cheats(&base_path, 2, 100)
}

fn find_cheats(path: &[Pos], can_skip_length: usize, target_skip: usize) -> usize {
    let mut tot = 0;
    for (idx, pos) in path[..path.len() - target_skip].iter().enumerate() {
        tot += path[idx + 1 + target_skip..]
            .iter()
            .enumerate()
            .filter(|(l, p)| {
                let distance = (pos - *p).l1_norm() as usize;
                let reachable_by_skip = distance > 1 && distance <= can_skip_length;
                let enough_saved = *l + 1 >= distance;
                reachable_by_skip && enough_saved
            })
            .count();
    }

    tot
}

pub fn part2((grid, start, end): (Grid, Pos, Pos)) -> usize {
    let (base_path, _) = dijkstra(&start, |p| successors(p, &grid), |p| *p == end).unwrap();
    find_cheats(&base_path, 20, 100)
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
        let (base_path, _) = dijkstra(&start, |p| successors(p, &grid), |p| *p == end).unwrap();
        let res = find_cheats(&base_path, 2, 2);
        assert_eq!(res, 44);
        let res_p2 = find_cheats(&base_path, 20, 50);
        assert_eq!(res_p2, 285);
    }
}
