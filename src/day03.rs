pub fn parse_input(input: &str) -> Vec<u8> {
    vec![0]
}

pub fn part1(input: Vec<u8>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        let input = indoc! {
            ""
        };
        let expected = vec![0];
        assert_eq!(parse_input(input), expected);
    }
}
