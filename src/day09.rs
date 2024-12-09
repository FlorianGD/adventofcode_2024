use std::ops::Range;

type Values = Vec<(usize, Range<usize>)>;
type Blanks = Vec<Range<usize>>;

pub fn parse_input(input: &str) -> (Values, Blanks) {
    let mut positions = vec![];
    let mut blanks = vec![];
    let mut current_pos = 0;
    for (i, c) in input.trim_end().chars().enumerate() {
        let val = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            positions.push((i / 2, current_pos..(current_pos + val)));
        } else {
            blanks.push(current_pos..(current_pos + val));
        }
        current_pos += val;
    }
    (positions, blanks)
}

pub fn part1((values, blanks): (Values, Blanks)) -> usize {
    let values_lengths: usize = values.iter().map(|(_, r)| r.len()).sum();
    // all the indices < values_length will not change
    let mut final_values: Values = values
        .clone()
        .into_iter()
        .filter(|(_, r)| r.end < values_lengths)
        .collect();
    let mut values: Values = values
        .clone()
        .into_iter()
        .filter(|(_, r)| r.end > values_lengths)
        .collect();
    for blank_range in blanks {
        if blank_range.start > values_lengths {
            if !values.is_empty() {
                final_values.extend(values);
            }
            break;
        }
        let mut current_position = blank_range.start;
        let mut len_to_fill = blank_range.len();
        if len_to_fill == 0 {
            continue;
        }
        while len_to_fill > 0 {
            if values.is_empty() {
                break;
            }
            let (value, range) = values.pop().unwrap();
            if range.len() <= len_to_fill {
                final_values.push((value, current_position..(current_position + range.len())));
                len_to_fill -= range.len();
                current_position += range.len();
            } else {
                final_values.push((value, current_position..(current_position + len_to_fill)));
                current_position += len_to_fill;
                values.push((
                    value,
                    current_position..(current_position + range.len() - len_to_fill),
                ));
                len_to_fill = 0;
            }
        }
    }
    final_values
        .into_iter()
        .map(|(val, r)| val * (r.sum::<usize>()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse_input() {
        let (values, blanks) = parse_input(INPUT);
        assert_eq!(values.len(), 10);
        assert_eq!(blanks.len(), 9);
        assert_eq!(values[0], (0, 0..2));
        assert_eq!(values[9], (9, 40..42));
        assert_eq!(blanks[0], 2..5);
        assert_eq!(blanks[8], 40..40);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 1928);
    }
}
