use num::Complex;
use rustc_hash::FxHashMap as HashMap;

type Grid = HashMap<Complex<isize>, char>;

const DIRECTIONS: [Complex<isize>; 8] = [
    Complex::new(0, 1),
    Complex::new(0, -1),
    Complex::new(1, 0),
    Complex::new(1, 1),
    Complex::new(1, -1),
    Complex::new(-1, 0),
    Complex::new(-1, 1),
    Complex::new(-1, -1),
];

pub fn parse_input(input: &str) -> (Vec<Complex<isize>>, Grid) {
    let mut xs = Vec::new();
    let mut grid = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Complex::new(x as isize, y as isize);
            if c == 'X' {
                xs.push(pos);
            }
            grid.insert(pos, c);
        }
    }

    (xs, grid)
}

fn check_xmas(x_pos: Complex<isize>, direction: Complex<isize>, grid: &Grid) -> bool {
    for (k, c) in "MAS".chars().enumerate() {
        let new_pos = x_pos + (k as isize + 1) * direction;
        if let Some(val) = grid.get(&new_pos) {
            if *val != c {
                return false;
            }
            continue;
        }
        return false;
    }
    true
}

fn check_all_directions(x_pos: Complex<isize>, grid: &Grid) -> usize {
    DIRECTIONS
        .into_iter()
        .filter(|&direction| check_xmas(x_pos, direction, grid))
        .count()
}

pub fn part1((xs, grid): (Vec<Complex<isize>>, Grid)) -> usize {
    xs.into_iter()
        .map(|x_pos| check_all_directions(x_pos, &grid))
        .sum()
}

pub fn parse_input_p2(input: &str) -> (Vec<Complex<isize>>, Grid) {
    let mut a_s = Vec::new();
    let mut grid = HashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Complex::new(x as isize, y as isize);
            if c == 'A' {
                a_s.push(pos);
            }
            grid.insert(pos, c);
        }
    }

    (a_s, grid)
}

fn check_cross_mas(a_pos: Complex<isize>, grid: &Grid) -> bool {
    let top_left = a_pos + Complex::new(-1, -1);
    let bottom_right = a_pos + Complex::new(1, 1);
    let top_right = a_pos + Complex::new(1, -1);
    let bottom_left = a_pos + Complex::new(-1, 1);

    // '@' here denotes a subpattern
    // zip allows use to check both values
    // not sure if this is clearer than the bare match on the previous commit
    if let Some((val @ ('M' | 'S'), val2 @ ('M' | 'S'))) =
        grid.get(&top_left).zip(grid.get(&bottom_right))
    {
        // val and val2 are either 'M' or 'S', we are ok if they are different
        if val == val2 {
            return false;
        }
    } else {
        // val and val2 are either None or not in "MS"
        return false;
    }
    if let Some((val @ ('M' | 'S'), val2 @ ('M' | 'S'))) =
        grid.get(&top_right).zip(grid.get(&bottom_left))
    {
        if val == val2 {
            return false;
        }
    } else {
        return false;
    }
    true
}

pub fn part2((a_s, grid): (Vec<Complex<isize>>, Grid)) -> usize {
    a_s.into_iter()
        .filter(|&a_pos| check_cross_mas(a_pos, &grid))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "
    };

    #[test]
    fn test_parse_input() {
        let (xs, grid) = parse_input(INPUT);
        assert_eq!(xs.len(), INPUT.chars().filter(|c| *c == 'X').count());
        assert_eq!(xs[0], Complex::new(4, 0));
        assert_eq!(grid[&Complex::new(0, 0)], 'M');
        assert_eq!(grid[&Complex::new(9, 9)], 'X');
    }

    #[test]
    fn test_check_xmas() {
        let (_xs, grid) = parse_input(INPUT);
        let x_pos = Complex::new(5, 0);
        assert!(check_xmas(x_pos, Complex::new(1, 0), &grid));
        assert!(!check_xmas(x_pos, Complex::new(1, 1), &grid));
        let x_pos = Complex::new(6, 5);
        assert!(check_xmas(x_pos, Complex::new(-1, -1), &grid))
    }

    #[test]
    fn test_check_all_directions() {
        let (_xs, grid) = parse_input(INPUT);
        let x_pos = Complex::new(3, 9);
        assert_eq!(check_all_directions(x_pos, &grid), 2);
    }

    #[test]
    fn test_part1() {
        let (xs, grid) = parse_input(INPUT);
        assert_eq!(part1((xs, grid)), 18);
    }

    #[test]
    fn test_check_cross_mass() {
        let (_a_s, grid) = parse_input_p2(INPUT);
        assert!(check_cross_mas(Complex::new(2, 1), &grid));
    }

    #[test]
    fn test_part2() {
        let (a_s, grid) = parse_input_p2(INPUT);
        assert_eq!(part2((a_s, grid)), 9);
    }
}
