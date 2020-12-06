use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Histogram {
    pub answers: HashMap<char, u8>,
    pub people: u8,
}

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Result<Vec<Histogram>, ParseIntError> {
    Ok(input
        .trim()
        .split_terminator("\n\n")
        .map(|l| {
            let mut answers = HashMap::new();
            for c in l.chars().filter(|c| c.is_alphabetic()) {
                *answers.entry(c).or_insert(0) += 1;
            }
            Histogram {
                answers,
                people: l.lines().count() as u8,
            }
        })
        .collect())
}

#[aoc(day6, part1)]
pub fn part1(passes: &[Histogram]) -> usize {
    passes.iter().map(|group| group.answers.len()).sum()
}

#[aoc(day6, part2)]
pub fn part2(passes: &[Histogram]) -> usize {
    passes
        .iter()
        .map(|group| {
            group
                .answers
                .iter()
                .filter(|(_, &c)| c == group.people)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b";

    const SAMPLE2: &str = "abcx
    abcy
    abcz";

    #[test]
    fn sample1() {
        let parsed = parse_input_day6(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 11);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day6(SAMPLE2).unwrap();
        assert_eq!(part1(&parsed), 6);
    }

    #[test]
    fn sample3() {
        let parsed = parse_input_day6(SAMPLE).unwrap();
        assert_eq!(part2(&parsed), 6);
    }
}
