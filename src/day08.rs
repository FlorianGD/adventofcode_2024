use itertools::Itertools;
use num::Complex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Grid = HashMap<char, Vec<Pos>>;

pub fn parse_input(input: &str) -> (Grid, isize, isize) {
    let mut grid: Grid = HashMap::default();
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                c => {
                    grid.entry(c)
                        .or_default()
                        .push(Complex::new(i as isize, j as isize));
                }
            }
        }
    }
    (grid, width, height)
}

pub fn part1((grid, width, height): (Grid, isize, isize)) -> usize {
    let mut antinodes = HashSet::default();
    for positions in grid.values() {
        for v in positions.iter().combinations(2) {
            let p1 = v[0];
            let p2 = v[1];
            let diff = p1 - p2;
            antinodes.insert(p2 - diff);
            antinodes.insert(p1 + diff);
        }
    }
    antinodes
        .iter()
        .filter(|p| p.re >= 0 && p.re < width && p.im >= 0 && p.im < height)
        .count()
}

pub fn part2((grid, width, height): (Grid, isize, isize)) -> usize {
    let mut antinodes = HashSet::default();
    for positions in grid.values() {
        for v in positions.iter().combinations(2) {
            let p1 = v[0];
            let p2 = v[1];
            let diff = p1 - p2;
            // positive
            let mut k = 0;
            while (p1 + diff * k).re < width
                && (p1 + diff * k).im < height
                && (p1 + diff * k).re >= 0
                && (p1 + diff * k).im >= 0
            {
                antinodes.insert(p1 + diff * k);
                k += 1;
            }
            // negative
            k = 0;
            while (p2 + diff * k).re < width
                && (p2 + diff * k).im < height
                && (p2 + diff * k).re >= 0
                && (p2 + diff * k).im >= 0
            {
                antinodes.insert(p2 + diff * k);
                k -= 1;
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "
    };

    #[test]
    fn test_parse_input() {
        let (grid, width, height) = parse_input(INPUT);
        assert_eq!(width, 12);
        assert_eq!(height, 12);
        assert_eq!(grid.len(), 2);
        assert_eq!(
            grid[&'0'],
            vec![
                Complex::new(8, 1),
                Complex::new(5, 2),
                Complex::new(7, 3),
                Complex::new(4, 4)
            ]
        )
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 14);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(input), 34);
    }
}
