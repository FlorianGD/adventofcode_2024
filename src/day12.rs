use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use num::Complex;
use rustc_hash::FxBuildHasher; // used to pop from the hashmap

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
type FxIndexSet<K> = IndexSet<K, FxBuildHasher>;

type Grid = FxIndexMap<Pos, char>;
type Pos = Complex<isize>;

const DIRECTIONS: [Pos; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

pub fn parse_input(input: &str) -> Grid {
    let mut grid = FxIndexMap::default();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            grid.insert(Complex::new(i as isize, j as isize), c);
        }
    }
    grid
}

fn possible_nexts<'a>(
    pos: &'a Pos,
    grid: &'a Grid,
    target: &'a char,
) -> impl Iterator<Item = Pos> + use<'a> {
    DIRECTIONS
        .into_iter()
        .filter(move |dir| grid.get(&(pos + dir)) == Some(target))
        .map(move |dir| pos + dir)
}

pub fn part1(grid: Grid) -> usize {
    let mut mutable_grid = grid.clone();
    let mut price = 0;
    while let Some((pos, val)) = mutable_grid.pop() {
        let mut seen = FxIndexSet::from_iter([pos]);
        let mut current_perimeter = 0;
        let mut current_area = 0;
        let mut stack = FxIndexSet::from_iter([pos]);
        while let Some(p) = stack.pop() {
            seen.insert(p);
            current_area += 1;
            current_perimeter += 4;
            for neighbor in possible_nexts(&p, &grid, &val) {
                current_perimeter -= 1;
                if !seen.contains(&neighbor) {
                    stack.insert(neighbor);
                    mutable_grid.swap_remove(&neighbor);
                }
            }
        }
        price += current_area * current_perimeter;
    }
    price
}

fn compute_sides(positions: &FxIndexSet<Pos>) -> usize {
    let min_re = positions.iter().min_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let max_re = positions.iter().max_by(|a, b| a.re.cmp(&b.re)).unwrap().re;
    let min_im = positions.iter().min_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    let max_im = positions.iter().max_by(|a, b| a.im.cmp(&b.im)).unwrap().im;
    let mut nb_sides = 0;
    for i in min_re..=max_re {
        let mut slice = positions
            .iter()
            .filter(|p| p.re == i && !positions.contains(&(*p + Complex::new(1, 0))))
            .collect_vec();
        if slice.is_empty() {
            continue;
        }
        nb_sides += 1;

        slice.sort_by(|a, b| a.im.cmp(&b.im));
        for (p1, p2) in slice.iter().tuple_windows() {
            if p1.im + 1 < p2.im {
                nb_sides += 1;
            }
        }
    }
    for i in min_re..=max_re {
        let mut slice = positions
            .iter()
            .filter(|p| p.re == i && !positions.contains(&(*p + Complex::new(-1, 0))))
            .collect_vec();
        if slice.is_empty() {
            continue;
        }
        nb_sides += 1;

        slice.sort_by(|a, b| a.im.cmp(&b.im));
        for (p1, p2) in slice.iter().tuple_windows() {
            if p1.im + 1 < p2.im {
                nb_sides += 1;
            }
        }
    }

    for j in min_im..=max_im {
        let mut slice = positions
            .iter()
            .filter(|p| p.im == j && !positions.contains(&(*p + Complex::new(0, 1))))
            .collect_vec();
        if slice.is_empty() {
            continue;
        }
        nb_sides += 1;

        slice.sort_by(|a, b| a.re.cmp(&b.re));
        for (p1, p2) in slice.iter().tuple_windows() {
            if p1.re + 1 < p2.re {
                nb_sides += 1;
            }
        }
    }

    for j in min_im..=max_im {
        let mut slice = positions
            .iter()
            .filter(|p| p.im == j && !positions.contains(&(*p + Complex::new(0, -1))))
            .collect_vec();
        if slice.is_empty() {
            continue;
        }
        nb_sides += 1;

        slice.sort_by(|a, b| a.re.cmp(&b.re));
        for (p1, p2) in slice.iter().tuple_windows() {
            if p1.re + 1 < p2.re {
                nb_sides += 1;
            }
        }
    }
    nb_sides
}
pub fn part2(grid: Grid) -> usize {
    let mut mutable_grid = grid.clone();
    let mut shapes = vec![];
    while let Some((pos, val)) = mutable_grid.pop() {
        let mut seen = FxIndexSet::from_iter([pos]);
        let mut stack = FxIndexSet::from_iter([pos]);
        while let Some(p) = stack.pop() {
            seen.insert(p);
            for neighbor in possible_nexts(&p, &grid, &val) {
                // current_perimeter -= 1;
                if !seen.contains(&neighbor) {
                    stack.insert(neighbor);
                    mutable_grid.swap_remove(&neighbor);
                }
            }
        }
        shapes.push(seen.clone());
    }
    shapes.iter().map(|p| p.len() * compute_sides(p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE"
    };

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert_eq!(grid[&Complex::new(0, 0)], 'R');
        assert_eq!(grid[&Complex::new(4, 0)], 'I');
        assert_eq!(grid[&Complex::new(0, 8)], 'M');
    }

    #[test]
    fn test_part1() {
        // let input = indoc! {
        //     "OOOOO
        //      OXOXO
        //      OOOOO
        //      OXOXO
        //      OOOOO"
        // };
        // assert_eq!(part1(parse_input(input)), 36 * 21 + 4 * 4);
        let grid = parse_input(INPUT);
        assert_eq!(part1(grid), 1930);
    }

    #[test]
    fn test_compute_sides_one_point() {
        let s1 = FxIndexSet::from_iter([Complex::new(0, 0)]);
        assert_eq!(compute_sides(&s1), 4);
    }

    #[test]
    fn test_compute_sides_two_points() {
        assert_eq!(
            compute_sides(&FxIndexSet::from_iter([
                Complex::new(0, 0),
                Complex::new(1, 0)
            ])),
            4
        );
    }

    #[test]
    fn test_compute_sides_four_points_in_square() {
        assert_eq!(
            compute_sides(&FxIndexSet::from_iter([
                Complex::new(0, 0),
                Complex::new(1, 0),
                Complex::new(0, 1),
                Complex::new(1, 1),
            ])),
            4
        );
    }

    #[test]
    fn test_compute_sides_three_points() {
        assert_eq!(
            compute_sides(&FxIndexSet::from_iter([
                Complex::new(0, 0),
                Complex::new(1, 0),
                Complex::new(0, 1),
            ])),
            6
        );
    }

    #[test]
    fn test_part2() {
        // let input = indoc! {
        //     "OOOOO
        //      OXOXO
        //      OOOOO
        //      OXOXO
        //      OOOOO"
        // };
        // assert_eq!(part1(parse_input(input)), 36 * 21 + 4 * 4);
        let grid = parse_input(INPUT);
        // &grid);
        assert_eq!(part2(grid), 1206);
    }
}
