use std::str::FromStr;

use winnow::{
    ascii::{digit1, newline},
    combinator::{delimited, preceded, terminated},
    token::one_of,
    PResult, Parser,
};

fn x(input: &mut &str) -> PResult<usize> {
    delimited(('X', one_of(['+', '='])), digit1, ", ")
        .parse_to()
        .parse_next(input)
}
fn y(input: &mut &str) -> PResult<usize> {
    preceded(('Y', one_of(['+', '='])), digit1)
        .parse_to()
        .parse_next(input)
}

fn button_a(input: &mut &str) -> PResult<(usize, usize)> {
    preceded("Button A: ", (x, y)).parse_next(input)
}

fn button_b(input: &mut &str) -> PResult<(usize, usize)> {
    preceded("Button B: ", (x, y)).parse_next(input)
}

fn prize(input: &mut &str) -> PResult<(usize, usize)> {
    preceded("Prize: ", (x, y)).parse_next(input)
}

fn parse_block(input: &mut &str) -> PResult<((usize, usize), (usize, usize), (usize, usize))> {
    (
        terminated(button_a, newline),
        terminated(button_b, newline),
        prize,
    )
        .parse_next(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    fn new(a: (usize, usize), b: (usize, usize), prize: (usize, usize)) -> Self {
        Machine { a, b, prize }
    }

    fn determinant(&self) -> isize {
        (self.a.0 * self.b.1) as isize - (self.b.0 * self.a.1) as isize
    }

    fn solve(&self) -> Option<isize> {
        let d = self.determinant();
        if d == 0 {
            return None;
        }
        let mut na = (self.b.1 * self.prize.0) as isize - (self.b.0 * self.prize.1) as isize;
        if na % d != 0 {
            return None;
        } else {
            na /= d;
        }

        let mut nb = (self.a.0 * self.prize.1) as isize - (self.a.1 * self.prize.0) as isize;
        if nb % d != 0 {
            return None;
        } else {
            nb /= d;
        }
        Some(3 * na + nb)
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s;
        if let Ok((a, b, prize)) = parse_block(&mut input) {
            Ok(Machine::new(a, b, prize))
        } else {
            Err(format!("Could not parse block {s}"))
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|block| block.parse().unwrap())
        .collect()
}

pub fn part1(machines: Vec<Machine>) -> isize {
    machines.iter().filter_map(|m| m.solve()).sum()
}

pub fn part2(machines: Vec<Machine>) -> isize {
    machines
        .iter()
        .map(|m| {
            Machine::new(
                m.a,
                m.b,
                (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
            )
        })
        .filter_map(|m| m.solve())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279"
    };

    #[test]
    fn test_parse_input() {
        let machines = parse_input(INPUT);
        assert_eq!(machines[0], Machine::new((94, 34), (22, 67), (8400, 5400)));
    }
    #[test]
    fn test_part1() {
        let machines = parse_input(INPUT);
        assert_eq!(part1(machines), 480);
    }
}
