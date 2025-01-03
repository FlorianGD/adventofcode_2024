use adventofcode2024::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20,
};

use anyhow::Result;
use aoc_next::{aoc_main, failable_parser, parser, solution, solver, Aoc};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2024,
    solutions: &[
        solution! {1, failable_parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, failable_parser!{ day01::parse_input }, solver!{ day01::part2 }},
        solution! {2, parser!{ day02::parse_input }, solver!{ day02::part1 }},
        solution! {2, parser!{ day02::parse_input }, solver!{ day02::part2 }},
        solution! {3, parser!{ day03::parse_input }, solver!{ day03::part1 }},
        solution! {3, parser!{ day03::parse_input_p2 }, solver!{ day03::part1 }},
        solution! {4, parser!{ day04::parse_input }, solver!{ day04::part1 }},
        solution! {4, parser!{ day04::parse_input_p2 }, solver!{ day04::part2 }},
        solution! {5, parser!{ day05::parse_input }, solver!{ day05::part1 }},
        solution! {5, parser!{ day05::parse_input }, solver!{ day05::part2 }},
        solution! {6, parser!{ day06::parse_input }, solver!{ day06::part1 }},
        solution! {6, parser!{ day06::parse_input }, solver!{ day06::part2 }},
        solution! {7, parser!{ day07::parse_input }, solver!{ day07::part1 }},
        solution! {7, parser!{ day07::parse_input }, solver!{ day07::part2 }},
        solution! {8, parser!{ day08::parse_input }, solver!{ day08::part1 }},
        solution! {8, parser!{ day08::parse_input }, solver!{ day08::part2 }},
        solution! {9, parser!{ day09::parse_input }, solver!{ day09::part1 }},
        solution! {9, parser!{ day09::parse_input }, solver!{ day09::part2 }},
        solution! {10, parser!{ day10::parse_input }, solver!{ day10::part1 }},
        solution! {10, parser!{ day10::parse_input }, solver!{ day10::part2 }},
        solution! {11, parser!{ day11::parse_input }, solver!{ day11::part1 }},
        solution! {11, parser!{ day11::parse_input }, solver!{ day11::part2 }},
        solution! {12, parser!{ day12::parse_input }, solver!{ day12::part1 }},
        solution! {13, parser!{ day13::parse_input }, solver!{ day13::part1 }},
        solution! {13, parser!{ day13::parse_input }, solver!{ day13::part2 }},
        solution! {14, parser!{ day14::parse_input }, solver!{ day14::part1 }},
        solution! {14, parser!{ day14::parse_input }, solver!{ day14::part2 }},
        solution! {15, parser!{ day15::parse_input }, solver!{ day15::part1 }},
        solution! {15, parser!{ day15::parse_input_p2 }, solver!{ day15::part2 }},
        solution! {16, parser!{ day16::parse_input }, solver!{ day16::part1 }},
        solution! {16, parser!{ day16::parse_input }, solver!{ day16::part2 }},
        solution! {18, parser!{ day18::parse_input }, solver!{ day18::part1 }},
        solution! {18, parser!{ day18::parse_input }, solver!{ day18::part2 }},
        solution! {19, parser!{ day19::parse_input }, solver!{ day19::part1 }},
        solution! {19, parser!{ day19::parse_input }, solver!{ day19::part2 }},
        solution! {17, parser!{ day17::parse_input }, solver!{ day17::part1 }},
        solution! {17, parser!{ day17::parse_input }, solver!{ day17::part2 }},
        solution! {20, parser!{ day20::parse_input }, solver!{ day20::part1 }},
        solution! {20, parser!{ day20::parse_input }, solver!{ day20::part1_2 }},
        solution! {20, parser!{ day20::parse_input }, solver!{ day20::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
