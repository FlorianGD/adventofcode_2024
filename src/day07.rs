pub fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (val, rest) = line.split_once(": ").unwrap();
            let val = val.parse().unwrap();
            let rest = rest.split(" ").map(|s| s.parse().unwrap()).collect();
            (val, rest)
        })
        .collect()
}

fn check_valid(target: usize, values: &[usize], current_val: usize) -> bool {
    if current_val > target {
        return false;
    }
    if values.is_empty() {
        return current_val == target;
    }
    let first_val = values[0];
    check_valid(target, &values[1..], first_val + current_val)
        || check_valid(target, &values[1..], first_val * current_val)
}

pub fn part1(input: Vec<(usize, Vec<usize>)>) -> usize {
    input
        .iter()
        .filter(|(target, values)| check_valid(*target, &values[1..], values[0]))
        .map(|(target, _)| target)
        .sum()
}

fn check_valid_p2(target: usize, values: &[usize], current_val: usize) -> bool {
    if current_val > target {
        return false;
    }
    if values.is_empty() {
        return current_val == target;
    }
    let first_val = values[0];
    check_valid_p2(target, &values[1..], first_val + current_val)
        || check_valid_p2(target, &values[1..], first_val * current_val)
        || check_valid_p2(
            target,
            &values[1..],
            (current_val.to_string() + &first_val.to_string())
                .parse()
                .unwrap(),
        )
}

pub fn part2(input: Vec<(usize, Vec<usize>)>) -> usize {
    input
        .iter()
        .filter(|(target, values)| check_valid_p2(*target, &values[1..], values[0]))
        .map(|(target, _)| target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
        "
    };

    #[test]
    fn test_parse_input() {
        let input = parse_input(INPUT);
        assert_eq!(input.len(), 9);
        assert_eq!(input[0].0, 190);
        assert_eq!(input[0].1, vec![10, 19]);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 3749);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(input), 11387);
    }
}
