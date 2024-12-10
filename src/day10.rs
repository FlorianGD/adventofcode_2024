use num::Complex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Grid = HashMap<Pos, u32>;

const DIRECTIONS: [Pos; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

pub fn parse_input(input: &str) -> (Grid, Vec<Pos>, Vec<Pos>) {
    let mut grid = HashMap::default();
    let mut zeros = vec![];
    let mut nines = vec![];
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let pos = Complex::new(i as isize, j as isize);
            match c {
                '0' => zeros.push(pos),
                '9' => nines.push(pos),
                _ => (),
            }
            grid.insert(pos, c.to_digit(10).unwrap());
        }
    }
    (grid, zeros, nines)
}

fn possible_nexts(pos: Pos, grid: &Grid) -> Vec<Pos> {
    let target_val = grid[&pos] + 1;
    DIRECTIONS
        .into_iter()
        .filter_map(|dir| {
            if let Some(p) = grid.get(&(pos + dir)) {
                if *p == target_val {
                    Some(pos + dir)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

pub fn part1((grid, zeros, nines): (Grid, Vec<Pos>, Vec<Pos>)) -> usize {
    let mut total_scores = 0;
    for zero in zeros {
        let mut reached = HashSet::default();
        let mut stack = vec![zero];
        while let Some(pos) = stack.pop() {
            if nines.contains(&pos) {
                reached.insert(pos);
            } else {
                stack.extend(possible_nexts(pos, &grid));
            }
        }
        total_scores += reached.len();
    }
    total_scores
}

pub fn part2((grid, zeros, nines): (Grid, Vec<Pos>, Vec<Pos>)) -> usize {
    let mut total_scores = 0;
    for zero in zeros {
        let mut stack = vec![zero];
        while let Some(pos) = stack.pop() {
            if nines.contains(&pos) {
                total_scores += 1;
            }
            stack.extend(possible_nexts(pos, &grid));
        }
    }
    total_scores
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use num::Zero;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
    "89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    "
        };

    #[test]
    fn test_parse_input() {
        let (grid, zeros, nines) = parse_input(INPUT);
        assert_eq!(grid[&Complex::zero()], 8);
        assert_eq!(grid[&Complex::new(1, 0)], 9);
        assert_eq!(grid[&Complex::new(0, 1)], 7);
        assert_eq!(zeros.len(), 9);
        assert!(zeros.contains(&Complex::new(2, 0)));
        assert_eq!(nines.len(), 7);
        assert!(nines.contains(&Complex::new(1, 0)));
    }

    #[test]
    fn test_possible_next() {
        let (grid, zeros, _) = parse_input(INPUT);
        assert_eq!(zeros[0], Complex::new(2, 0));
        let nexts = possible_nexts(zeros[0], &grid);
        assert_eq!(
            HashSet::from_iter(nexts),
            HashSet::from_iter([Complex::new(2, 1), Complex::new(3, 0)])
        );
        let nexts = possible_nexts(Complex::new(1, 0), &grid);
        assert!(nexts.is_empty());
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(input), 81);
    }
}
