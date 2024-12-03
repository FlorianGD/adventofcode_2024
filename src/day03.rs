use regex::Regex;

pub fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    input
        .lines()
        .flat_map(|line| re.captures_iter(line).map(|c| c.extract()))
        .map(|(_, [a, b])| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

pub fn part1(input: Vec<(u32, u32)>) -> u32 {
    input.iter().fold(0, |acc, (a, b)| acc + a * b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    const INPUT: &str = indoc! {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
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
}
