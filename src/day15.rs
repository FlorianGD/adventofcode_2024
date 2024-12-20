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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ElementLarge {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

type Grid2 = HashMap<Pos, ElementLarge>;

pub fn parse_input_p2(input: &str) -> (Pos, Grid2, Directions) {
    let (grid_str, instructions) = input.split_once("\n\n").unwrap();
    let mut grid = HashMap::default();
    let mut robot_pos = Complex::zero();
    for (j, line) in grid_str.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let pos_left = Complex::new(2 * i as isize, j as isize);
            let pos_right = Complex::new((2 * i + 1) as isize, j as isize);
            let (elem1, elem2) = match c {
                '#' => (ElementLarge::Wall, ElementLarge::Wall),
                'O' => (ElementLarge::BoxLeft, ElementLarge::BoxRight),
                '.' => (ElementLarge::Empty, ElementLarge::Empty),
                '@' => {
                    robot_pos = pos_left;
                    (ElementLarge::Empty, ElementLarge::Empty)
                }
                c => panic!("Unexpected character {}", c),
            };
            grid.insert(pos_left, elem1);
            grid.insert(pos_right, elem2);
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

fn _print_grid(grid: &Grid2, robot_pos: &Pos) {
    if grid.is_empty() {
        return;
    }
    let x_max = grid.iter().map(|(p, _)| p.re).max().unwrap();
    let y_max = grid.iter().map(|(p, _)| p.im).max().unwrap();
    for j in 0..=y_max {
        for i in 0..=x_max {
            let p = Complex::new(i, j);
            if &p == robot_pos {
                print!("@");
                continue;
            }
            let c = match grid.get(&p) {
                None => 'x',
                Some(ElementLarge::Empty) => '.',
                Some(ElementLarge::Wall) => '#',
                Some(ElementLarge::BoxLeft) => '[',
                Some(ElementLarge::BoxRight) => ']',
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

/// Give the Left side of each box that can move
fn can_move_p2(pos: &Pos, direction: &Direction, grid: &Grid2) -> Option<HashSet<Pos>> {
    let new_pos = pos + direction.val();
    match (grid[&new_pos], direction) {
        (ElementLarge::Empty, _) => Some(HashSet::default()),
        (ElementLarge::Wall, _) => None,
        (e, Direction::Left | Direction::Right) => {
            if let Some(mut r) = can_move_p2(&new_pos, direction, grid) {
                if e == ElementLarge::BoxLeft {
                    r.insert(new_pos);
                }
                Some(r)
            } else {
                None
            }
        }
        // the difficulty arises when we move a block up or down
        (ElementLarge::BoxLeft, _) => {
            let can_move_left_side = can_move_p2(&new_pos, direction, grid);
            let can_move_right_side =
                can_move_p2(&(new_pos + Direction::Right.val()), direction, grid);
            match (can_move_left_side, can_move_right_side) {
                (None, _) => None,
                (_, None) => None,
                (Some(mut vl), Some(vr)) => {
                    vl.extend(vr);
                    vl.insert(new_pos);
                    Some(vl)
                }
            }
        }
        (ElementLarge::BoxRight, _) => {
            let can_move_left_side =
                can_move_p2(&(new_pos + Direction::Left.val()), direction, grid);
            let can_move_right_side = can_move_p2(&(new_pos), direction, grid);
            match (can_move_left_side, can_move_right_side) {
                (None, _) => None,
                (_, None) => None,
                (Some(mut vl), Some(vr)) => {
                    vl.extend(vr);
                    vl.insert(new_pos + Direction::Left.val());
                    Some(vl)
                }
            }
        }
    }
}

/// compute the new column after being pushed
fn build_mini_grid(
    // initial_pos: &Pos,
    boxes_to_move: &HashSet<Pos>,
    direction: &Direction,
) -> Grid2 {
    let mut mini_grid = Grid2::default();
    for box_left in boxes_to_move {
        mini_grid.insert(*box_left + direction.val(), ElementLarge::BoxLeft);
        mini_grid.insert(
            *box_left + direction.val() + Direction::Right.val(),
            ElementLarge::BoxRight,
        );
    }
    // _print_grid(&mini_grid, initial_pos);
    mini_grid
}

/// update the grid from final_pos to initial_pos
fn update_grid_p2(
    // initial_pos: &Pos,
    final_positions: &HashSet<Pos>,
    direction: &Direction,
    grid: &mut Grid2,
) {
    let mini_grid: Grid2 = build_mini_grid(
        //initial_pos,
        final_positions,
        direction,
    );
    for (pos, elem) in mini_grid.clone() {
        grid.insert(pos, elem);
        if !mini_grid.contains_key(&(pos - direction.val())) {
            grid.insert(pos - direction.val(), ElementLarge::Empty);
        }
    }
}

pub fn part2((robot_pos, grid, directions): (Pos, Grid2, Directions)) -> isize {
    let mut current_pos = robot_pos;
    let mut grid = grid;
    let num_boxes = grid
        .iter()
        .filter(|(_, v)| *v == &ElementLarge::BoxLeft)
        .count();
    for direction in directions {
        // println!("{direction:#?}");
        if let Some(values_final_pos) = can_move_p2(&current_pos, &direction, &grid) {
            update_grid_p2(
                //&current_pos,
                &values_final_pos,
                &direction,
                &mut grid,
            );
            current_pos += direction.val();
        }
        if grid
            .iter()
            .filter(|(_, v)| *v == &ElementLarge::BoxLeft)
            .count()
            != num_boxes
        {
            _print_grid(&grid, &current_pos);
            panic!()
        }
        // _print_grid(&grid, &current_pos);
    }
    grid.into_iter()
        .filter(|(_, e)| *e == ElementLarge::BoxLeft)
        .map(|(p, _)| p.re + 100 * p.im)
        .sum()
    // 1403280 too low
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

    #[test]
    fn test_parse_input_p2() {
        let (robot_pos, grid, directions) = parse_input_p2(INPUT);
        // It should be this grid
        //   00000000001111111111
        //   01234567890123456789
        // 0 ####################
        // 1 ##....[]....[]..[]##
        // 2 ##............[]..##
        // 3 ##..[][]....[]..[]##
        // 4 ##....[]@.....[]..##
        // 5 ##[]##....[]......##
        // 6 ##[]....[]....[]..##
        // 7 ##..[][]..[]..[][]##
        // 8 ##........[]......##
        // 9 ####################
        assert_eq!(robot_pos, Complex::new(8, 4));
        assert_eq!(grid[&robot_pos], ElementLarge::Empty);
        assert_eq!(grid[&Complex::zero()], ElementLarge::Wall);
        assert_eq!(grid[&Complex::new(1, 1)], ElementLarge::Wall);
        assert_eq!(grid[&Complex::new(6, 1)], ElementLarge::BoxLeft);
        assert_eq!(grid[&Complex::new(7, 1)], ElementLarge::BoxRight);
        assert_eq!(directions.len(), 700);
        assert_eq!(directions[0], Direction::Left);
    }

    #[test]
    fn test_can_move_p2() {
        let (robot_pos, grid, _) = parse_input_p2(INPUT);
        let direction = Direction::Up;
        let can_move_up = can_move_p2(&robot_pos, &direction, &grid);
        assert_eq!(can_move_up, Some(HashSet::default()));
        let can_move_left = can_move_p2(&robot_pos, &Direction::Left, &grid);
        assert_eq!(
            can_move_left,
            Some(HashSet::from_iter([Complex::new(6, 4)]))
        );
    }

    #[test]
    fn test_can_move_p2_up_with_box_left() {
        let (_, grid, _) = parse_input_p2(INPUT);
        let direction = Direction::Up;
        let pos = Complex::new(6, 5);
        assert_eq!(grid[&(pos + direction.val())], ElementLarge::BoxLeft);
        let can_move_up = can_move_p2(&pos, &direction, &grid);
        assert_eq!(
            can_move_up,
            Some(HashSet::from_iter([Complex::new(6, 4), Complex::new(6, 3)]))
        );
    }

    #[test]
    fn test_can_move_p2_up_with_box_right() {
        let (_, grid, _) = parse_input_p2(INPUT);
        let direction = Direction::Up;
        let pos = Complex::new(7, 5);
        assert_eq!(grid[&(pos + direction.val())], ElementLarge::BoxRight);
        let can_move_up = can_move_p2(&pos, &direction, &grid);
        assert_eq!(
            can_move_up,
            Some(HashSet::from_iter([Complex::new(6, 4), Complex::new(6, 3)]))
        );
    }

    #[test]
    fn test_update_grid_p2() {
        let (_, mut grid, _) = parse_input_p2(INPUT);
        let direction = Direction::Up;
        let pos = Complex::new(7, 5);

        //   00000000001111111111
        //   01234567890123456789
        // 0 ####################
        // 1 ##....[]....[]..[]##
        // 2 ##............[]..##
        // 3 ##..[][]....[]..[]##
        // 4 ##....[]......[]..##
        // 5 ##[]##.@..[]......##
        // 6 ##[]....[]....[]..##
        // 7 ##..[][]..[]..[][]##
        // 8 ##........[]......##
        // 9 ####################
        let can_move_up = can_move_p2(&pos, &direction, &grid).unwrap();

        update_grid_p2(
            //&pos,
            &can_move_up,
            &direction,
            &mut grid,
        );

        // It should be this grid
        //   00000000001111111111
        //   01234567890123456789
        // 0 ####################
        // 1 ##....[]....[]..[]##
        // 2 ##....[]......[]..##
        // 3 ##..[][]....[]..[]##
        // 4 ##.....@......[]..##
        // 5 ##[]##....[]......##
        // 6 ##[]....[]....[]..##
        // 7 ##..[][]..[]..[][]##
        // 8 ##........[]......##
        // 9 ####################
        assert_eq!(grid[&Complex::new(7, 4)], ElementLarge::Empty);
        assert_eq!(grid[&Complex::new(7, 3)], ElementLarge::BoxRight);
        assert_eq!(grid[&Complex::new(6, 3)], ElementLarge::BoxLeft);
        assert_eq!(grid[&Complex::new(7, 2)], ElementLarge::BoxRight);
        assert_eq!(grid[&Complex::new(6, 2)], ElementLarge::BoxLeft);
    }

    #[test]
    fn test_update_grid_p2_down_box_left() {
        let (_, mut grid, _) = parse_input_p2(INPUT);
        let direction = Direction::Down;
        let pos = Complex::new(14, 5);

        //   00000000001111111111
        //   01234567890123456789
        // 0 ####################
        // 1 ##....[]....[]..[]##
        // 2 ##............[]..##
        // 3 ##..[][]....[]..[]##
        // 4 ##....[]......[]..##
        // 5 ##[]##....[]..@...##
        // 6 ##[]....[]....[]..##
        // 7 ##..[][]..[]..[][]##
        // 8 ##........[]......##
        // 9 ####################
        let can_move_down = can_move_p2(&pos, &direction, &grid).unwrap();

        update_grid_p2(
            //&pos,
            &can_move_down,
            &direction,
            &mut grid,
        );

        // It should be this grid
        //   00000000001111111111
        //   01234567890123456789
        // 0 ####################
        // 1 ##....[]....[]..[]##
        // 2 ##............[]..##
        // 3 ##..[][]....[]..[]##
        // 4 ##....[]......[]..##
        // 5 ##[]##....[]......##
        // 6 ##[]....[]....@...##
        // 7 ##..[][]..[]..[][]##
        // 8 ##........[]..[]..##
        // 9 ####################
        assert_eq!(grid[&Complex::new(14, 5)], ElementLarge::Empty);
        assert_eq!(grid[&Complex::new(14, 6)], ElementLarge::Empty);
        assert_eq!(grid[&Complex::new(15, 7)], ElementLarge::BoxRight);
        assert_eq!(grid[&Complex::new(14, 7)], ElementLarge::BoxLeft);
        assert_eq!(grid[&Complex::new(15, 8)], ElementLarge::BoxRight);
        assert_eq!(grid[&Complex::new(14, 8)], ElementLarge::BoxLeft);
        _print_grid(&grid, &(pos + direction.val()));
    }

    #[test]
    fn test_part2() {
        //         let input = "#######
        // #...#.#
        // #.....#
        // #..OO@#
        // #..O..#
        // #.....#
        // #######

        // <vv<<^^<<^^>v";
        let (robot_pos, grid, directions) = parse_input_p2(INPUT);
        _print_grid(&grid, &robot_pos);
        assert_eq!(part2((robot_pos, grid, directions)), 9021);
        // panic!();
    }

    #[test]
    fn test_p2_complex_move() {
        //   012345678
        // 0 ##.......
        // 1 ##.[][][]
        // 2 ##[][][].
        // 3 ##.[][][]
        // 4 ##..@..[]
        let robot_pos = Complex::new(4, 4);
        let mut grid: Grid2 = HashMap::from_iter([
            (Complex::new(0, 0), ElementLarge::Wall),
            (Complex::new(0, 1), ElementLarge::Wall),
            (Complex::new(0, 2), ElementLarge::Wall),
            (Complex::new(0, 3), ElementLarge::Wall),
            (Complex::new(0, 4), ElementLarge::Wall),
            (Complex::new(1, 0), ElementLarge::Wall),
            (Complex::new(1, 1), ElementLarge::Wall),
            (Complex::new(1, 2), ElementLarge::Wall),
            (Complex::new(1, 3), ElementLarge::Wall),
            (Complex::new(1, 4), ElementLarge::Wall),
            (Complex::new(2, 0), ElementLarge::Empty),
            (Complex::new(2, 1), ElementLarge::Empty),
            (Complex::new(2, 2), ElementLarge::BoxLeft),
            (Complex::new(2, 3), ElementLarge::Empty),
            (Complex::new(2, 4), ElementLarge::Empty),
            (Complex::new(3, 0), ElementLarge::Empty),
            (Complex::new(3, 1), ElementLarge::BoxLeft),
            (Complex::new(3, 2), ElementLarge::BoxRight),
            (Complex::new(3, 3), ElementLarge::BoxLeft),
            (Complex::new(3, 4), ElementLarge::Empty),
            (Complex::new(4, 0), ElementLarge::Empty),
            (Complex::new(4, 1), ElementLarge::BoxRight),
            (Complex::new(4, 2), ElementLarge::BoxLeft),
            (Complex::new(4, 3), ElementLarge::BoxRight),
            (Complex::new(4, 4), ElementLarge::Empty),
            (Complex::new(5, 0), ElementLarge::Empty),
            (Complex::new(5, 1), ElementLarge::BoxLeft),
            (Complex::new(5, 2), ElementLarge::BoxRight),
            (Complex::new(5, 3), ElementLarge::BoxLeft),
            (Complex::new(5, 4), ElementLarge::Empty),
            (Complex::new(6, 0), ElementLarge::Empty),
            (Complex::new(6, 1), ElementLarge::BoxRight),
            (Complex::new(6, 2), ElementLarge::BoxLeft),
            (Complex::new(6, 3), ElementLarge::BoxRight),
            (Complex::new(6, 4), ElementLarge::Empty),
            (Complex::new(7, 0), ElementLarge::Empty),
            (Complex::new(7, 1), ElementLarge::BoxLeft),
            (Complex::new(7, 2), ElementLarge::BoxRight),
            (Complex::new(7, 3), ElementLarge::BoxLeft),
            (Complex::new(7, 4), ElementLarge::BoxLeft),
            (Complex::new(8, 0), ElementLarge::Empty),
            (Complex::new(8, 1), ElementLarge::BoxRight),
            (Complex::new(8, 2), ElementLarge::Empty),
            (Complex::new(8, 3), ElementLarge::BoxRight),
            (Complex::new(8, 4), ElementLarge::BoxRight),
        ]);
        _print_grid(&grid, &robot_pos);
        let direction = Direction::Up;
        let final_positions = can_move_p2(&robot_pos, &direction, &grid).unwrap();
        println!("{:#?}", final_positions);
        update_grid_p2(
            //&robot_pos,
            &final_positions,
            &direction,
            &mut grid,
        );
        _print_grid(&grid, &(robot_pos + direction.val()));
        // panic!();
    }
}
