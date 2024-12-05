use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Rules = HashMap<usize, HashSet<usize>>;
type Pages = Vec<Vec<usize>>;

pub fn parse_input(input: &str) -> (Rules, Pages) {
    let (rules_list, prints) = input.split_once("\n\n").unwrap();
    let pages = prints
        .lines()
        .map(|line| line.split(",").map(|p| p.parse().unwrap()).collect())
        .collect();

    let mut rules = HashMap::default();

    for line in rules_list.lines() {
        let (before, after) = line.split_once("|").unwrap();
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        rules
            .entry(after)
            .or_insert_with(HashSet::default)
            .insert(before);
    }
    (rules, pages)
}

fn check_page(page: &[usize], rules: &Rules) -> bool {
    for i in 0..page.len() {
        let pages_after: HashSet<usize> = page.iter().skip(i).copied().collect();
        if let Some(rule) = rules.get(&page[i]) {
            if rule.intersection(&pages_after).next().is_some() {
                return false;
            }
        }
    }
    true
}

pub fn part1((rules, pages): (Rules, Pages)) -> usize {
    pages
        .into_iter()
        .filter(|page| check_page(page, &rules))
        .map(|page| page[page.len() / 2])
        .sum()
}

/// Fix the order and return the middle value
fn fix_order(page: &[usize], rules: &Rules) -> usize {
    let page_set: HashSet<usize> = page.iter().copied().collect();
    *page
        .iter()
        .find(|&p| {
            rules
                .get(p) // funnily, this is not needed for the real input, only for the tests
                .unwrap_or(&HashSet::default())
                .intersection(&page_set)
                .count()
                == page_set.len() / 2
        })
        .unwrap()
}

pub fn part2((rules, pages): (Rules, Pages)) -> usize {
    pages
        .into_iter()
        .filter(|page| !check_page(page, &rules))
        .map(|page| fix_order(&page, &rules))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"
    };

    #[test]
    fn test_parse_input() {
        let (rules, pages) = parse_input(INPUT);
        assert_eq!(rules[&75], HashSet::from_iter([97]));
        assert_eq!(pages.len(), 6);
        assert_eq!(pages[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_check_page() {
        let (rules, pages) = parse_input(INPUT);
        assert!(check_page(&pages[0], &rules));
        assert!(check_page(&pages[1], &rules));
        assert!(check_page(&pages[2], &rules));
        assert!(!check_page(&pages[3], &rules));
        assert!(!check_page(&pages[4], &rules));
        assert!(!check_page(&pages[5], &rules));
    }

    #[test]
    fn test_part1() {
        let (rules, pages) = parse_input(INPUT);
        assert_eq!(part1((rules, pages)), 143);
    }

    #[test]
    fn test_part2() {
        let (rules, pages) = parse_input(INPUT);
        assert_eq!(part2((rules, pages)), 123);
    }
}
