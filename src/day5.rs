use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub struct BoardingPass {
    row: usize,
    col: usize,
}

impl BoardingPass {
    pub fn from_binary(binary: &str) -> Self {
        assert_eq!(binary.len(), 10);

        let mut row_seacher = BinarySearch::new(127);
        let mut col_seacher = BinarySearch::new(7);

        let (rows, cols) = binary.split_at(7);

        for c in rows.chars() {
            row_seacher.step(c);
        }

        for c in cols.chars() {
            col_seacher.step(c);
        }

        assert_eq!(row_seacher.0, row_seacher.1);
        assert_eq!(col_seacher.0, col_seacher.1);

        Self {
            row: row_seacher.0,
            col: col_seacher.0,
        }
    }

    pub fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

pub struct BinarySearch(usize, usize);

impl BinarySearch {
    pub fn new(max: usize) -> Self {
        Self(0, max)
    }

    pub fn step(&mut self, c: char) {
        let (lower, upper) = match c {
            'B' | 'R' => (self.0 + ((self.1 - self.0) / 2) + 1, self.1),
            'F' | 'L' => (self.0, self.1 - ((self.1 - self.0) / 2) - 1),
            _ => panic!("Unexpected character"),
        };

        self.0 = lower;
        self.1 = upper;
    }
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Result<Vec<BoardingPass>, ParseIntError> {
    Ok(input
        .trim()
        .lines()
        .map(|l| BoardingPass::from_binary(l.trim()))
        .collect())
}

#[aoc(day5, part1)]
pub fn part1(passes: &[BoardingPass]) -> usize {
    passes
        .iter()
        .map(|pass| pass.seat_id())
        .max()
        .expect("No boarding passes found")
}

#[aoc(day5, part2)]
pub fn part2(passes: &[BoardingPass]) -> usize {
    let mut seat_ids: Vec<_> = passes.iter().map(|pass| pass.seat_id()).collect();
    seat_ids.sort();

    seat_ids
        .windows(2)
        .find(|&x| x[0] + 1 != x[1])
        .expect("My seat is nowhere to be found!")[0]
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "FBFBBFFRLR";

    #[test]
    fn input_parser() {
        assert_eq!(
            parse_input_day5(SAMPLE),
            Ok(vec![BoardingPass { row: 44, col: 5 }])
        );
    }

    #[test]
    fn sample1() {
        let parsed = parse_input_day5(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 357);
    }

    #[test]
    fn pass_1() {
        let pass = BoardingPass::from_binary("BFFFBBFRRR");

        assert_eq!(pass.row, 70);
        assert_eq!(pass.col, 7);
        assert_eq!(part1(&[pass]), 567);
    }

    #[test]
    fn pass_2() {
        let pass = BoardingPass::from_binary("FFFBBBFRRR");

        assert_eq!(pass.row, 14);
        assert_eq!(pass.col, 7);
        assert_eq!(part1(&[pass]), 119);
    }

    #[test]
    fn pass_3() {
        let pass = BoardingPass::from_binary("BBFFBBFRLL");

        assert_eq!(pass.row, 102);
        assert_eq!(pass.col, 4);
        assert_eq!(part1(&[pass]), 820);
    }

    #[test]
    fn pass_all() {
        let pass1 = BoardingPass::from_binary("BFFFBBFRRR");
        let pass2 = BoardingPass::from_binary("FFFBBBFRRR");
        let pass3 = BoardingPass::from_binary("BBFFBBFRLL");

        assert_eq!(part1(&[pass1, pass2, pass3]), 820);
    }
}
