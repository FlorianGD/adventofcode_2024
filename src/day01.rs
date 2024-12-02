use counter::Counter;

pub fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once("   ").unwrap();
            (p1.parse().unwrap(), p2.parse().unwrap())
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
    let (l1, l2): (Vec<i32>, Vec<i32>) = input.iter().cloned().unzip();
    let c1 = l1.iter().collect::<Counter<_, i32>>();
    let c2 = l2.iter().collect::<Counter<_, i32>>();
    c1.iter()
        .map(|(k, v)| c2.get(k).unwrap_or(&0) * v * *k)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "};
        let vec = parse_input(input);
        assert_eq!(vec, vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)]);
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
        "3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "};
        let vec = parse_input(input);
        let result = part1(vec);
        assert_eq!(result, 11);
    }
    #[test]
    fn test_part2() {
        let input = indoc! {
        "3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "};
        let vec = parse_input(input);
        let result = part2(vec);
        assert_eq!(result, 31);
    }
}
