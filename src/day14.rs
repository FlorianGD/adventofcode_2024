use rustc_hash::FxHashSet as HashSet;
use std::str::FromStr;

use num::Complex;
use winnow::{
    ascii::digit1,
    combinator::{preceded, separated_pair},
    PResult, Parser,
};

use crate::parsers::neg_num;

type Pos = Complex<isize>;

fn p(input: &mut &str) -> PResult<(isize, isize)> {
    preceded(
        "p=",
        separated_pair(digit1.parse_to(), ',', digit1.parse_to()),
    )
    .parse_next(input)
}
fn v(input: &mut &str) -> PResult<(isize, isize)> {
    preceded(
        " v=",
        separated_pair(neg_num::<isize>, ',', neg_num::<isize>),
    )
    .parse_next(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    p: Pos,
    v: Pos,
}

impl Robot {
    fn new(p: Pos, v: Pos) -> Self {
        Robot { p, v }
    }

    fn move_n_steps(&mut self, n: isize, max_x: isize, max_y: isize) {
        let new_re = (self.p.re + n * self.v.re).rem_euclid(max_x);
        let new_im = (self.p.im + n * self.v.im).rem_euclid(max_y);
        self.p = Complex::new(new_re, new_im);
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s;
        if let Ok((p, v)) = (p, v).parse_next(&mut input) {
            Ok(Robot::new(Complex::new(p.0, p.1), Complex::new(v.0, v.1)))
        } else {
            Err(format!("Could not parse line {s}"))
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: Vec<Robot>) -> usize {
    let mut robots = input.clone();
    let max_x = 101;
    let max_y = 103;
    for robot in &mut robots {
        robot.move_n_steps(100, max_x, max_y);
    }
    let top_left = robots
        .iter()
        .filter(|r| r.p.re < max_x / 2 && r.p.im < max_y / 2)
        .count();
    let top_right = robots
        .iter()
        .filter(|r| r.p.re > max_x / 2 && r.p.im < max_y / 2)
        .count();
    let bottom_left = robots
        .iter()
        .filter(|r| r.p.re < max_x / 2 && r.p.im > max_y / 2)
        .count();
    let bottom_right = robots
        .iter()
        .filter(|r| r.p.re > max_x / 2 && r.p.im > max_y / 2)
        .count();
    top_left * top_right * bottom_left * bottom_right
}

fn _print_grid(grid: &HashSet<Complex<isize>>, max_x: isize, max_y: isize) {
    for i in 0..max_x {
        for j in 0..max_y {
            if grid.contains(&Complex::new(i, j)) {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!()
}

pub fn part2(input: Vec<Robot>) -> isize {
    let mut robots = input.clone();
    let max_x = 101;
    let max_y = 103;
    let steps = 7623;
    for robot in &mut robots {
        robot.move_n_steps(steps, max_x, max_y);
    }
    // let grid: HashSet<Complex<isize>> = HashSet::from_iter(robots.iter().map(|r| r.p));
    // _print_grid(&grid, max_x, max_y);
    // loop {
    //     println!("{steps}");
    //     for robot in &mut robots {
    //         robot.move_n_steps(1, max_x, max_y);
    //     }
    //     // let's look for a lot of robots on the same line, let's say more than 20
    //     let mut d = 0;
    //     for i in 0..max_y {
    //         let grid: HashSet<Complex<isize>> = HashSet::from_iter(robots.iter().map(|r| r.p));
    //         let c = grid.iter().filter(|p| p.re == i).count();
    //         if c > 20 {
    //             d += 1;
    //         }
    //     }
    //     if d > 4 {
    //         _print_grid(&grid, max_x, max_y);
    //     }
    //     steps += 1;
    // }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3"
    };

    #[test]
    fn test_parse_input() {
        let robots = parse_input(INPUT);
        assert_eq!(
            robots[0],
            Robot::new(Complex::new(0, 4), Complex::new(3, -3))
        );
    }

    #[test]
    fn test_move() {
        let mut robot: Robot = "p=2,4 v=2,-3".parse().unwrap();
        // 1 step in a 11x7 grid
        robot.move_n_steps(1, 11, 7);
        assert_eq!(robot.p, Complex::new(4, 1));
        robot.move_n_steps(4, 11, 7);
        assert_eq!(robot.p, Complex::new(1, 3));
    }

    #[test]
    fn test_part1() {
        let robots = parse_input(INPUT);
        assert_eq!(part1(robots), 12);
    }
}
