use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

/// Position as (down, right)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(usize, usize);

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    trees: Vec<Position>,
    end: Position,
}

#[derive(Debug)]
pub struct Toboggan {
    location: Position,
}

impl Toboggan {
    pub fn new() -> Self {
        Self {
            location: Position(0, 0),
        }
    }

    pub fn spawn(pos: Position) -> Self {
        Self { location: pos }
    }

    pub fn sleigh(&mut self, rel_pos: Position, map: &Map) -> bool {
        self.location += rel_pos;
        self.location.1 %= map.end.1;

        map.trees.contains(&self.location)
    }

    pub fn at_bottom(&self, map: &Map) -> bool {
        self.location.0 >= map.end.0
    }
}

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Result<Map, ParseIntError> {
    let trees = input
        .lines()
        .enumerate()
        .map(|(down, l)| {
            let mut trees = Vec::new();
            for (right, _) in l.trim().char_indices().filter(|(_, c)| c == &'#') {
                trees.push(Position(down, right))
            }
            trees
        })
        .flatten()
        .collect();

    let mut iter = input.lines();
    let end_right = iter.next().unwrap_or("").trim().len();
    let end_down = iter.count() + 1;

    Ok(Map {
        trees,
        end: Position(end_down, end_right),
    })
}

#[aoc(day3, part1)]
pub fn part1(map: &Map) -> usize {
    let mut toboggan = Toboggan::new();

    let mut trees_hit = 0;
    while !toboggan.at_bottom(map) {
        if toboggan.sleigh(Position(1, 3), map) {
            trees_hit += 1;
        }
    }
    trees_hit
}

#[aoc(day3, part2)]
pub fn part2(map: &Map) -> usize {
    let slopes = vec![
        Position(1, 1),
        Position(1, 3),
        Position(1, 5),
        Position(1, 7),
        Position(2, 1),
    ];

    slopes
        .iter()
        .map(|p| {
            let mut toboggan = Toboggan::new();

            let mut trees_hit = 0;
            while !toboggan.at_bottom(map) {
                if toboggan.sleigh(*p, map) {
                    trees_hit += 1;
                }
            }
            trees_hit
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    #[test]
    fn input_parser() {
        let trees = vec![
            Position(0, 2),
            Position(0, 3),
            Position(1, 0),
            Position(1, 4),
            Position(1, 8),
            Position(2, 1),
            Position(2, 6),
            Position(2, 9),
            Position(3, 2),
            Position(3, 4),
            Position(3, 8),
            Position(3, 10),
            Position(4, 1),
            Position(4, 5),
            Position(4, 6),
            Position(4, 9),
            Position(5, 2),
            Position(5, 4),
            Position(5, 5),
            Position(6, 1),
            Position(6, 3),
            Position(6, 5),
            Position(6, 10),
            Position(7, 1),
            Position(7, 10),
            Position(8, 0),
            Position(8, 2),
            Position(8, 3),
            Position(8, 7),
            Position(9, 0),
            Position(9, 4),
            Position(9, 5),
            Position(9, 10),
            Position(10, 1),
            Position(10, 4),
            Position(10, 8),
            Position(10, 10),
        ];

        let end = Position(11, 11);
        assert_eq!(parse_input_day3(SAMPLE), Ok(Map { trees, end }));
    }

    #[test]
    fn sample1() {
        let parsed = parse_input_day3(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 7);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day3(SAMPLE).unwrap();
        assert_eq!(part2(&parsed), 336);
    }
}
