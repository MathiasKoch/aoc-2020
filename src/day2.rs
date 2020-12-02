use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
    min: usize,
    max: usize,
    character: char,
}

pub enum Policy {
    SledRental,
    TobogganRental,
}

impl PasswordPolicy {
    pub fn from_input(policy: &str) -> Self {
        let mut pol_iter = policy.split_whitespace().into_iter();

        let mut limits_iter = pol_iter.next().expect("No limits present?!").split('-');

        Self {
            min: limits_iter.next().expect("no min?").parse().unwrap(),
            max: limits_iter.next().expect("no max?").parse().unwrap(),
            character: pol_iter
                .last()
                .expect("no character?")
                .chars()
                .last()
                .expect(""),
        }
    }

    pub fn validate(&self, policy: Policy, password: &str) -> bool {
        match policy {
            Policy::SledRental => {
                let cnt = password.chars().filter(|&c| c == self.character).count();
                self.min <= cnt && cnt <= self.max
            }
            Policy::TobogganRental => {
                let mut char_iter = password
                    .char_indices()
                    .filter(|(i, _)| i + 1 == self.min || i + 1 == self.max)
                    .map(|(_, c)| c);

                let first = char_iter
                    .next()
                    .and_then(|c| Some(c == self.character))
                    .unwrap_or(false);

                let second = char_iter
                    .next()
                    .and_then(|c| Some(c == self.character))
                    .unwrap_or(false);
                first ^ second
            }
        }
    }
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<(PasswordPolicy, String)>, ParseIntError> {
    input
        .lines()
        .map(|l| {
            let mut pol_iter = l.splitn(2, ": ").into_iter();

            let policy =
                PasswordPolicy::from_input(pol_iter.next().expect("No password policy present?!"));
            let password = String::from(pol_iter.next().expect("No password present?!"));

            Ok((policy, password))
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(password_pairs: &[(PasswordPolicy, String)]) -> usize {
    password_pairs
        .iter()
        .filter(|(policy, password)| policy.validate(Policy::SledRental, password))
        .count()
}

#[aoc(day2, part2)]
pub fn part2(password_pairs: &[(PasswordPolicy, String)]) -> usize {
    password_pairs
        .iter()
        .filter(|(policy, password)| policy.validate(Policy::TobogganRental, password))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    #[test]
    fn input_parser() {
        let a = (
            PasswordPolicy {
                min: 1,
                max: 3,
                character: '1',
            },
            String::from("abcde"),
        );
        let b = (
            PasswordPolicy {
                min: 1,
                max: 3,
                character: 'b',
            },
            String::from("cdefg"),
        );
        let c = (
            PasswordPolicy {
                min: 2,
                max: 9,
                character: 'c',
            },
            String::from("ccccccccc"),
        );
        assert_eq!(parse_input_day2(SAMPLE), Ok(vec![a, b, c]));
    }

    #[test]
    fn sample1() {
        let parsed = parse_input_day2(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 2);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day2(SAMPLE).unwrap();
        assert_eq!(part2(&parsed), 1);
    }
}
