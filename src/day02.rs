use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    levels: Vec<i32>,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Ok(Record { levels })
    }
}

impl Record {
    fn is_safe(&self) -> bool {
        self.levels
            .iter()
            .zip(self.levels.iter().skip(1))
            .all(|(a, b)| a > b && (a - b) >= 1 && (a - b) <= 3)
            || self
                .levels
                .iter()
                .zip(self.levels.iter().skip(1))
                .all(|(b, a)| a > b && (a - b) >= 1 && (a - b) <= 3)
    }

    fn is_safe_p2(&self) -> bool {
        if self.is_safe() {
            return true;
        };
        for i in 0..self.levels.len() {
            let mut l = self.levels.clone();
            l.remove(i);
            let r = Record { levels: l };
            if r.is_safe() {
                return true;
            }
        }
        false
    }
}

pub fn parse_input(input: &str) -> Vec<Record> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: Vec<Record>) -> usize {
    input.iter().filter(|r| r.is_safe()).count()
}

pub fn part2(input: Vec<Record>) -> usize {
    input.iter().filter(|r| r.is_safe_p2()).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "};
        let vec = parse_input(input);
        assert_eq!(vec.len(), 6);
        assert_eq!(
            vec[0],
            Record {
                levels: vec![7, 6, 4, 2, 1]
            }
        );
    }
    #[test]
    fn test_is_safe() {
        let r = Record {
            levels: vec![7, 6, 4, 2, 1],
        };
        assert!(r.is_safe());
        let r = Record {
            levels: vec![1, 2, 7, 8, 9],
        };
        assert!(!r.is_safe());
        let r = Record {
            levels: vec![1, 3, 2, 4, 5],
        };
        assert!(!r.is_safe());
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "};
        let vec = parse_input(input);
        assert_eq!(part1(vec), 2);
    }
    #[test]
    fn test_part2() {
        let input = indoc! {
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "};
        let vec = parse_input(input);
        assert_eq!(part2(vec), 4);
    }
}
