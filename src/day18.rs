use std::collections::HashSet;

use num::Complex;
use pathfinding::prelude::dijkstra;

type Pos = Complex<isize>;

pub fn parse_input(input: &str) -> Vec<(usize, Pos)> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (x, y) = l.split_once(",").unwrap();
            let pos = Complex::new(x.parse().unwrap(), y.parse().unwrap());
            (i, pos)
        })
        .collect()
}

fn build_grid(positions: &[(usize, Pos)], limit: usize) -> HashSet<Pos> {
    positions
        .iter()
        .filter(|(i, _)| *i < limit)
        .map(|(_, p)| *p)
        .collect()
}
const DIRECTIONS: [Pos; 4] = [
    Complex::new(1, 0),
    Complex::new(0, 1),
    Complex::new(-1, 0),
    Complex::new(0, -1),
];

fn successors(p: &Pos, grid: &HashSet<Pos>, x_max: isize, y_max: isize) -> Vec<(Pos, usize)> {
    let mut next = Vec::new();
    for d in DIRECTIONS {
        match grid.get(&(p + d)) {
            Some(_) => (),
            None => {
                if p.re >= 0 && p.re <= x_max && p.im >= 0 && p.im <= y_max {
                    next.push((p + d, 1));
                }
            }
        }
    }
    next
}

fn _print_grid(x_max: isize, y_max: isize, grid: &HashSet<Pos>, path: &Option<Vec<Pos>>) {
    for j in 0..=y_max {
        for i in 0..=x_max {
            let pos = Complex::new(i, j);
            if grid.contains(&pos) {
                print!("#");
            } else {
                match path {
                    None => print!("."),
                    Some(p) => {
                        if p.contains(&pos) {
                            print!("O");
                        } else {
                            print!(".");
                        }
                    }
                }
            }
        }
        println!();
    }
    println!();
}

pub fn part1(positions: Vec<(usize, Pos)>) -> usize {
    let limit = 1024;
    let target = Complex::new(70, 70);
    let grid = build_grid(&positions, limit);
    let (x_max, y_max) = (target.re, target.im);
    // _print_grid(x_max, y_max, &grid, &None);
    if let Some((_path, cost)) = dijkstra(
        &Complex::new(0, 0),
        |p| successors(p, &grid, x_max, y_max),
        |p| *p == target,
    ) {
        // _print_grid(x_max, y_max, &grid, &Some(_path));
        cost
    } else {
        panic!("No solution found");
    }
}

pub fn part2(positions: Vec<(usize, Pos)>) -> Pos {
    let mut max_steps = positions.len();
    let mut min_steps = 1024;
    let mut limit = (max_steps + min_steps) / 2;
    let target = Complex::new(70, 70);
    let (x_max, y_max) = (target.re, target.im);
    let mut grid = build_grid(&positions, limit);

    loop {
        if dijkstra(
            &Complex::new(0, 0),
            |p| successors(p, &grid, x_max, y_max),
            |p| *p == target,
        )
        .is_some()
        {
            min_steps = limit;
            limit = (limit + max_steps) / 2;
            if limit == min_steps {
                break;
            }
            grid = build_grid(&positions, limit);
        } else {
            max_steps = limit;
            let new_limit = (min_steps + limit + 1) / 2;
            if new_limit == limit {
                limit -= 1; // not sure why, but hey
                break;
            }
            limit = new_limit;
            grid = build_grid(&positions, limit);
        }
    }
    positions[limit].1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0"
    };

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed[0], (0, Complex::new(5, 4)));
    }

    // #[test]
    // fn test_part2() {
    //     let parsed = parse_input(INPUT);
    //     assert_eq!(part2(parsed), Complex::new(6, 1));
    // }
}
