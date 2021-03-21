use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occupancy {
    Floor,
    Empty,
    Occupied,
}

impl Display for Occupancy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Floor => '.',
            Self::Empty => 'L',
            Self::Occupied => '#',
        };
        write!(f, "{}", c)
    }
}

impl Occupancy {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Unknown char"),
        }
    }
    pub fn swap(&mut self) {
        *self = match self {
            Self::Floor => Self::Floor,
            Self::Empty => Self::Occupied,
            Self::Occupied => Self::Empty,
        }
    }

    pub fn is_occupied(o: &Self) -> bool {
        matches!(o, Self::Occupied)
    }
}

/// Position as (row, col)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(usize, usize);

#[derive(Clone, PartialEq, Eq)]
pub struct SeatLayout {
    layout: HashMap<Position, Occupancy>,
}

impl Display for SeatLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bp = self.layout.keys().max().unwrap();

        for r in 0..=bp.0 {
            for c in 0..=bp.1 {
                write!(f, "{}", self.layout.get(&Position(r, c)).unwrap())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl SeatLayout {
    pub fn adjacent_occupied(sl: &Self, seat: &Position) -> usize {
        let row = seat.0;
        let col = seat.1;

        let positions = [
            Position(row.wrapping_sub(1), col.wrapping_sub(1)), // Left Up
            Position(row.wrapping_sub(1), col),                 // Up
            Position(row.wrapping_sub(1), col + 1),             // Right Up
            Position(row, col.wrapping_sub(1)),                 // Left
            Position(row, col + 1),                             // Right
            Position(row + 1, col.wrapping_sub(1)),             // Left Down
            Position(row + 1, col),                             // Down
            Position(row + 1, col + 1),                         // Right Down
        ];

        positions
            .iter()
            .filter_map(|p| sl.layout.get(p))
            .cloned()
            .filter(Occupancy::is_occupied)
            .count()
    }

    pub fn first_occupied(sl: &Self, seat: &Position) -> usize {
        let row = seat.0;
        let col = seat.1;

        let positions = [
            Position(row.wrapping_sub(1), col.wrapping_sub(1)), // Left Up
            Position(row.wrapping_sub(1), col),                 // Up
            Position(row.wrapping_sub(1), col + 1),             // Right Up
            Position(row, col.wrapping_sub(1)),                 // Left
            Position(row, col + 1),                             // Right
            Position(row + 1, col.wrapping_sub(1)),             // Left Down
            Position(row + 1, col),                             // Down
            Position(row + 1, col + 1),                         // Right Down
        ];

        positions
            .iter()
            .filter_map(|p| sl.layout.get(p))
            .cloned()
            .filter(Occupancy::is_occupied)
            .count()
    }

    /// Updates the current seating model, returning whether any seats have
    /// changed state.
    pub fn update<F>(&mut self, tolerance: usize, f: F) -> bool
    where
        F: Fn(&Self, &Position) -> usize,
    {
        let state = &self.clone();
        let mut changed = false;

        self.layout.iter_mut().for_each(|(s, o)| match o {
            Occupancy::Empty => {
                if f(state, s) == 0 {
                    o.swap();
                    changed |= true;
                }
            }
            Occupancy::Occupied => {
                if f(state, s) >= tolerance {
                    o.swap();
                    changed |= true;
                }
            }
            _ => {}
        });

        changed
    }

    pub fn occupied_seats(&self) -> usize {
        self.layout
            .values()
            .cloned()
            .filter(Occupancy::is_occupied)
            .count()
    }
}

impl FromStr for SeatLayout {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            layout: s
                .trim()
                .lines()
                .enumerate()
                .map(|(r, l)| {
                    let mut p = Vec::new();
                    for (c, o) in l.trim().char_indices() {
                        p.push((Position(r, c), Occupancy::new(o)));
                    }
                    p
                })
                .flatten()
                .collect(),
        })
    }
}

#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Result<SeatLayout, Box<dyn std::error::Error>> {
    input.parse()
}

#[aoc(day11, part1)]
pub fn part1(layout: &SeatLayout) -> usize {
    let mut lay = layout.clone();
    while lay.update(4, SeatLayout::adjacent_occupied) {
        // println!("After update:");
        // println!("{}\n\n", lay);
    }
    lay.occupied_seats()
}

#[aoc(day11, part2)]
pub fn part2(layout: &SeatLayout) -> usize {
    let mut lay = layout.clone();
    while lay.update(5, SeatLayout::first_occupied) {
        // println!("After update:");
        // println!("{}\n\n", lay);
    }
    lay.occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

    #[test]
    fn parsing() {
        let parsed = parse_input_day11(SAMPLE).unwrap();
        println!("{}", parsed);
        assert_eq!(parsed.layout.keys().len(), 100);
    }

    #[test]
    fn sample1() {
        let parsed = parse_input_day11(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 37);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day11(SAMPLE).unwrap();
        assert_eq!(part2(&parsed), 26);
    }
}
