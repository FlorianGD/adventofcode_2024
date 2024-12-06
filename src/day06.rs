use num::Complex;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Dir = Complex<isize>;
type Grid = HashMap<Pos, State>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Empty,
    Obstacle,
}

pub fn parse_input(input: &str) -> (Pos, Grid) {
    let mut start = Complex::default();
    let mut grid = HashMap::default();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let (pos, state) = match c {
                '^' => {
                    start = Complex::new(i as isize, j as isize);
                    (Complex::new(i as isize, j as isize), State::Empty)
                }
                '.' => (Complex::new(i as isize, j as isize), State::Empty),
                '#' => (Complex::new(i as isize, j as isize), State::Obstacle),
                _ => panic!("unexpected character"),
            };
            grid.insert(pos, state);
        }
    }
    (start, grid)
}

pub fn part1((start, grid): (Pos, Grid)) -> usize {
    // we are facing up, but the imaginary axis is backwards
    let mut direction = -Complex::i();
    let mut pos = start;
    let mut steps = HashSet::default();
    steps.insert(pos);
    while let Some(state) = grid.get(&(pos + direction)) {
        match state {
            State::Empty => {
                pos += direction;
                steps.insert(pos);
            }
            State::Obstacle => direction *= Complex::i(),
        }
    }
    steps.len()
}

fn print_grid_and_steps(grid: &Grid, steps: &HashSet<(Pos, Dir)>) {
    let size = grid.iter().map(|(a, _)| a.re).max().unwrap();
    for j in 0..=size {
        let mut s = String::new();
        for i in 0..=size {
            let pos = Complex::new(i, j);
            let c = match steps.iter().find(|(p, _d)| *p == pos) {
                Some(_) => 'X',
                None => match grid.get(&pos) {
                    Some(state) => match state {
                        State::Empty => '.',
                        State::Obstacle => '#',
                    },
                    None => ' ',
                },
            };
            s = format!("{}{}", s, c);
        }
        println!("{}", s);
    }
    println!();
}

fn find_loop(position: Pos, direction: Dir, steps: &HashSet<(Pos, Dir)>, grid: &Grid) -> bool {
    let mut steps = steps.to_owned();
    // we add a block at position + direction, so the new direction to check is rotated
    // by 90°
    let mut grid = grid.clone();
    grid.insert(position + direction, State::Obstacle);
    // println!("looking for a loop at {position} with {direction}");
    let mut direction = direction * Complex::i();
    let mut position = position;
    while let Some(state) = grid.get(&(position + direction)) {
        // println!("position {position}");
        match state {
            State::Obstacle => {
                // println!("Found an obstacle at {position}");
                // position -= direction;
                direction *= Complex::i();
                // println!("new position {position} new direction {direction}");
                // print_grid_and_steps(&grid, &steps;
            }
            State::Empty => {
                // did we reach this point with this direction already?
                if steps.contains(&(position, direction)) {
                    // println!("Found a loop at {position} with {direction}");
                    // print_grid_and_steps(&grid, &steps;
                    return true;
                }
                // we can continue
                steps.insert((position, direction));
                position += direction;
            }
        }
        // println!("new position {position}, new direction {direction}");
    }
    // println!("out of the grid at {position}");
    // println!("no loop found for {position} and {direction}");
    // print_grid_and_steps(&grid, &steps;
    false
}

pub fn part2((start, grid): (Pos, Grid)) -> usize {
    let mut direction = -Complex::i();
    let mut pos = start;
    let mut steps = HashSet::default();
    let mut blocks: HashSet<Complex<isize>> = HashSet::default();
    steps.insert((pos, direction));
    while let Some(state) = grid.get(&(pos + direction)) {
        match state {
            State::Empty => {
                if find_loop(pos, direction, &steps, &grid) {
                    blocks.insert(pos + direction);
                }
                pos += direction;
                steps.insert((pos, direction));
            }
            State::Obstacle => direction *= Complex::i(),
        }
    }
    blocks.len()
    // not 1643
    // 1325 too low
    // not 2019
    // not 1922
}

pub fn part2_old((start, grid): (Pos, Grid)) -> usize {
    // we are facing up, but the imaginary axis is backwards
    // println!("start {start}");
    let mut direction = -Complex::i();
    let mut pos = start;
    let mut steps = HashSet::default();
    let mut blocks: HashSet<Complex<isize>> = HashSet::default();
    steps.insert((pos, direction));
    // println!("Inserted {}, {}", pos, direction);

    while let Some(state) = grid.get(&(pos + direction)) {
        // println!(
        //     "Current pos {pos}, we get state {state:?} for {}",
        //     pos + direction
        // );
        match state {
            State::Empty => {
                // If we put a block  at pos+direction (which we can, it is empty), we
                // will rotate 90°. Let's check if we can reach a point already on the
                // path. If so, count it
                let block_position = pos + direction;
                // we cannot add a block even if the target location is empty if it is a
                // point that we already reached. If we reach the below formation going
                // up, we cannot add a block going down, because it would prevent us to
                // reach this point in the first place.
                // .#.
                // #.#
                // besides, we cannot add a point at the start
                if block_position == start {
                    println!("{block_position} was already reached with -{direction}");
                    pos += direction;
                    steps.insert((pos, direction));
                    continue;
                }

                if find_loop(pos, direction, &steps, &grid) {
                    blocks.insert(pos + direction);
                }

                pos += direction;
                steps.insert((pos, direction));
                // println!("Inserted {}, {}", pos, direction);
            }
            State::Obstacle => direction *= Complex::i(),
        }
    }
    // println!("{:?}", steps);
    blocks.len()
    // 1365 too low
    // not 5338
    // not 2019
    // not 1922
    // answer: 1796?
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
        "
    };

    #[test]
    fn test_parse_input() {
        let (start, grid) = parse_input(INPUT);
        assert_eq!(start, Complex::new(4, 6));
        assert_eq!(grid[&Complex::new(4, 6)], State::Empty);
        assert_eq!(grid[&Complex::new(4, 0)], State::Obstacle);
        assert_eq!(grid[&Complex::new(0, 0)], State::Empty);
    }

    #[test]
    fn test_part1() {
        let (start, grid) = parse_input(INPUT);
        assert_eq!(part1((start, grid)), 41);
    }

    #[test]
    fn test_part2() {
        let (start, grid) = parse_input(INPUT);
        assert_eq!(part2((start, grid)), 6);
    }
}
