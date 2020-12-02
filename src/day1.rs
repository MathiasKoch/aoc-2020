use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
pub fn part1(numbers: &[i32]) -> i32 {
    if let Some((a, b)) = find_numbers(numbers, &0) {
        return a * b;
    }
    0
}

#[aoc(day1, part2)]
pub fn part2(numbers: &[i32]) -> i32 {
    for start in numbers {
        if let Some((a, b)) = find_numbers(numbers, start) {
            return a * b * start;
        }
    }
    0
}

fn find_numbers(numbers: &[i32], start: &i32) -> Option<(i32, i32)> {
    for x in numbers.iter() {
        let y = 2020 - x - start;
        if numbers.contains(&y) {
            return Some((*x, y));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[i32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn sample1() {
        assert_eq!(part1(SAMPLE), 514579);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(SAMPLE), 241861950);
    }
}
