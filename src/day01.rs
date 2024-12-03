use anyhow::{Context, Result};
use counter::Counter;

pub fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once("   ").context("bad input")?;
            Ok((p1.parse()?, p2.parse()?))
        })
        .collect()
}

pub fn part1(input: Vec<(i32, i32)>) -> u32 {
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = input.iter().cloned().unzip();
    l1.sort();
    l2.sort();
    l1.iter().zip(l2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

pub fn part2(input: Vec<(i32, i32)>) -> i32 {
    let (c1, c2): (Counter<_, i32>, Counter<_, i32>) = input.iter().cloned().unzip();

    c1.iter()
        .map(|(k, v)| c2.get(k).unwrap_or(&0) * v * *k)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    const INPUT: &str = indoc! {
    "3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "};

    #[test]
    fn test_parse_input() {
        // the test fails if `unwrap` panics, which is what we want
        let vec = parse_input(INPUT).unwrap();
        assert_eq!(vec, vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)]);
    }

    #[test]
    fn test_part1() {
        let vec = parse_input(INPUT).unwrap();
        let result = part1(vec);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let vec = parse_input(INPUT).unwrap();
        let result = part2(vec);
        assert_eq!(result, 31);
    }
}
