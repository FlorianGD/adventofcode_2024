use memoize::memoize;
use rustc_hash::FxHashSet as HashSet;

pub fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut patterns: Vec<String> = patterns.split(", ").map(String::from).collect();
    patterns.sort_by_key(|s| std::cmp::Reverse(s.len()));

    let designs = designs.lines().map(String::from).collect();

    (patterns, designs)
}

#[memoize]
fn possible_design(design: String, patterns: Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    let possible_patterns: Vec<String> = patterns
        .iter()
        .filter(|p| design.contains(p.as_str()))
        .map(|p| p.to_string())
        .collect();
    if possible_patterns.is_empty() {
        return false;
    }
    let possible_letters: HashSet<char> =
        possible_patterns.iter().flat_map(|p| p.chars()).collect();
    let all_letters = design.chars().collect::<HashSet<char>>();
    if all_letters.difference(&possible_letters).count() != 0 {
        return false;
    }

    possible_patterns.clone().into_iter().any(|pattern| {
        if design.starts_with(&pattern) {
            let d = &design[pattern.len()..];
            possible_design(d.to_owned(), possible_patterns.clone())
        } else {
            false
        }
    })
}

pub fn part1((patterns, designs): (Vec<String>, Vec<String>)) -> usize {
    designs
        .into_iter()
        .filter(|design| possible_design(String::from(design), patterns.clone()))
        .count()
}

#[memoize]
fn all_possible_designs(design: String, patterns: Vec<String>) -> usize {
    if design.is_empty() {
        return 1;
    }
    let possible_patterns: Vec<String> = patterns
        .iter()
        .filter(|p| design.contains(p.as_str()))
        .map(|p| p.to_string())
        .collect();
    if possible_patterns.is_empty() {
        return 0;
    }
    let possible_letters: HashSet<char> =
        possible_patterns.iter().flat_map(|p| p.chars()).collect();
    let all_letters = design.chars().collect::<HashSet<char>>();
    if all_letters.difference(&possible_letters).count() != 0 {
        return 0;
    }

    let mut res = 0;
    for pattern in possible_patterns.clone() {
        if design.starts_with(&pattern) {
            let d = &design[pattern.len()..];
            res += all_possible_designs(d.to_owned(), possible_patterns.clone());
        }
    }
    res
}

pub fn part2((patterns, designs): (Vec<String>, Vec<String>)) -> usize {
    designs
        .into_iter()
        .map(|design| all_possible_designs(design, patterns.clone()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
    };

    #[test]
    fn test_parse_input() {
        let (patterns, designs) = parse_input(INPUT);
        assert_eq!(patterns.len(), 8);
        assert_eq!(patterns[0], String::from("bwu"));
        assert_eq!(designs.len(), 8);
    }

    #[test]
    fn test_part1() {
        let (patterns, designs) = parse_input(INPUT);
        assert_eq!(part1((patterns, designs)), 6);
        // panic!();
    }

    #[test]
    fn test_part2() {
        let (patterns, designs) = parse_input(INPUT);
        assert_eq!(part2((patterns, designs)), 16);
        // panic!();
    }
}
