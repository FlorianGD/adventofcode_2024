use rustc_hash::FxHashMap as HashMap;

pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn apply_rules(val: usize) -> Vec<usize> {
    if val == 0 {
        return vec![1];
    }
    let val_str = val.to_string();
    if val_str.len() % 2 == 0 {
        return vec![
            val_str[..val_str.len() / 2].parse().unwrap(),
            val_str[val_str.len() / 2..].parse().unwrap(),
        ];
    }
    vec![2024 * val]
}

pub fn part1(input: Vec<usize>) -> usize {
    let mut values = input;
    for _ in 0..25 {
        let mut new_vals = vec![];
        for val in values {
            new_vals.extend(apply_rules(val));
        }
        values = new_vals;
    }
    values.len()
}

pub fn part2(input: Vec<usize>) -> usize {
    let mut transforms: HashMap<usize, Vec<usize>> = HashMap::default();
    let mut counts: HashMap<usize, usize> = HashMap::default();
    for val in input {
        counts.insert(val, 1);
    }
    for _ in 0..75 {
        let old_counts = counts.clone();
        for (val, count) in old_counts {
            let new_vals = transforms
                .entry(val)
                .or_insert_with_key(|val| apply_rules(*val));
            counts.entry(val).and_modify(|e| *e -= count);
            for new_val in new_vals {
                counts
                    .entry(*new_val)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        }
        counts.retain(|_, &mut v| v > 0);
    }
    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = "125 17";

    #[test]
    fn test_parse_input() {
        let input = parse_input(INPUT);
        assert_eq!(input, vec![125, 17]);
    }

    #[test]
    fn test_apply_rule() {
        assert_eq!(apply_rules(0), vec![1]);
        assert_eq!(apply_rules(1), vec![2024]);
        assert_eq!(apply_rules(1000), vec![10, 0]);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 55312);
    }
}
