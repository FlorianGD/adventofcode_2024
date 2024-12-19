use std::collections::{HashMap, HashSet};

use num::{Complex, Zero};

type Pos = Complex<isize>;
type Grid = HashMap<Pos, Element>;
type Directions = Vec<Direction>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn val(&self) -> Complex<isize> {
        match self {
            Self::Left => Complex::new(-1, 0),
            Self::Right => Complex::new(1, 0),
            // imaginary axis is flipped
            Self::Up => Complex::new(0, -1),
            Self::Down => Complex::new(0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Element {
    Empty,
    Wall,
    Box,
}

pub fn parse_input(input: &str) -> (Pos, Grid, Directions) {
    let (grid_str, instructions) = input.split_once("\n\n").unwrap();
    let mut grid = HashMap::default();
    let mut robot_pos = Complex::zero();
    for (j, line) in grid_str.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let pos = Complex::new(i as isize, j as isize);
            let elem = match c {
                '#' => Element::Wall,
                'O' => Element::Box,
                '.' => Element::Empty,
                '@' => {
                    robot_pos = pos;
                    Element::Empty
                }
                c => panic!("Unexpected character {}", c),
            };
            grid.insert(pos, elem);
        }
    }

    let instructions = instructions.replace("\n", "");
    let directions = instructions
        .chars()
        .map(|c| match c {
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            c => panic!("Unexpected character {}", c),
        })
        .collect();
    (robot_pos, grid, directions)
}

fn can_move(pos: &Pos, direction: &Direction, grid: &Grid) -> Option<Pos> {
    let new_pos = pos + direction.val();
    match grid[&new_pos] {
        Element::Empty => Some(new_pos),
        Element::Wall => None,
        Element::Box => can_move(&new_pos, direction, grid),
    }
}

/// update the grid from final_pos to initial_pos
fn update_grid(initial_pos: &Pos, final_pos: &Pos, direction: &Direction, grid: &mut Grid) {
    let mut current_pos = *final_pos;
    while &current_pos != initial_pos {
        let new_val = grid[&(current_pos - direction.val())];
        grid.entry(current_pos).and_modify(|val| *val = new_val);
        current_pos -= direction.val();
    }
}

pub fn part1((robot_pos, grid, directions): (Pos, Grid, Directions)) -> isize {
    let mut current_pos = robot_pos;
    let mut grid = grid;
    for direction in directions {
        if let Some(final_pos) = can_move(&current_pos, &direction, &grid) {
            update_grid(&current_pos, &final_pos, &direction, &mut grid);
            current_pos += direction.val();
        }
    }
    grid.into_iter()
        .filter(|(_, e)| *e == Element::Box)
        .map(|(p, _)| p.re + 100 * p.im)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    };

    #[test]
    fn test_parse_input() {
        let (robot_pos, grid, directions) = parse_input(INPUT);
        assert_eq!(robot_pos, Complex::new(4, 4));
        assert_eq!(grid[&robot_pos], Element::Empty);
        assert_eq!(grid[&Complex::zero()], Element::Wall);
        assert_eq!(grid[&Complex::new(3, 1)], Element::Box);
        assert_eq!(directions.len(), 700);
        assert_eq!(directions[0], Direction::Left);
    }

    #[test]
    fn test_can_move() {
        let (robot_pos, grid, _directions) = parse_input(INPUT);
        // the robot in (1, 1) cannot move up
        assert_eq!(can_move(&Complex::new(1, 1), &Direction::Up, &grid), None);

        // the robot can move up
        assert_eq!(
            can_move(&robot_pos, &Direction::Up, &grid),
            Some(robot_pos + Direction::Up.val())
        );
        // the robot can move left, and it will move a box
        assert_eq!(
            can_move(&robot_pos, &Direction::Left, &grid),
            Some(robot_pos + Direction::Left.val() * 2)
        );
        // if the robot is in (4, 7), it can move left, and it will move 2 boxes
        assert_eq!(
            can_move(&Complex::new(4, 7), &Direction::Left, &grid),
            Some(Complex::new(4, 7) + Direction::Left.val() * 3)
        );
        // if the robot is in (4, 9), it cannot move right because of the boxes
        assert_eq!(
            can_move(&Complex::new(4, 9), &Direction::Right, &grid),
            None
        );
    }

    #[test]
    fn test_update_grid() {
        let (robot_pos, mut grid, directions) = parse_input(INPUT);
        let direction = directions[0];
        let new_pos = can_move(&robot_pos, &direction, &grid).unwrap();
        update_grid(&robot_pos, &new_pos, &direction, &mut grid);
        assert_eq!(grid[&(robot_pos + direction.val())], Element::Empty);
        assert_eq!(grid[&(robot_pos + 2 * direction.val())], Element::Box);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(input), 10092);
    }
}
