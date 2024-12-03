use regex::Regex;
use std::sync::LazyLock;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

pub fn parse_input(input: &str) -> Vec<(u32, u32)> {
    RE.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: Vec<(u32, u32)>) -> u32 {
    input.iter().fold(0, |acc, (a, b)| acc + a * b)
}

pub fn parse_input_p2(input: &str) -> Vec<(u32, u32)> {
    let l = input.replace("\n", "");
    l.split("don't()")
        .enumerate()
        .filter_map(|(i, s)| {
            if i == 0 {
                Some(parse_input(s))
            } else if let Some((_excluded, not_excluded)) = s.split_once("do()") {
                Some(parse_input(not_excluded))
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    const INPUT: &str = indoc! {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    };
    const INPUT_P2: &str = indoc! {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    };

    #[test]
    fn test_parse_input() {
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(parse_input(INPUT), expected);
    }

    #[test]
    fn test_part1() {
        let expected = 161;
        assert_eq!(part1(parse_input(INPUT)), expected);
    }

    #[test]
    fn test_parse_input_p2() {
        let expected = vec![(2, 4), (8, 5)];
        assert_eq!(parse_input_p2(INPUT_P2), expected);
    }
}
