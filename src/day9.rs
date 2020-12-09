use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

pub fn decode_xmas(numbers: &[usize], preamble: usize) -> usize {
    numbers
        .iter()
        .enumerate()
        .skip(preamble)
        .find(|(i, n)| find_numbers(&numbers[i - preamble..*i], n).is_none())
        .map(|(_, n)| *n)
        .expect("Failed to find a number!")
}

fn find_numbers(numbers: &[usize], n: &usize) -> Option<usize> {
    for x in numbers.iter() {
        if let Some(y) = n.checked_sub(*x) {
            if numbers.contains(&y) {
                return Some(*x);
            }
        }
    }
    None
}

pub fn encryption_weakness(numbers: &[usize], n: usize) -> usize {
    for i in 2..20 {
        if let Some(w) = numbers.windows(i).find(|&w| w.iter().sum::<usize>() == n) {
            return w.iter().min().unwrap() + w.iter().max().unwrap();
        }
    }
    panic!("Failed to find contiguous set with sum: {:?}", n);
}

#[aoc_generator(day9)]
fn parse_input_day9(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.trim().lines().map(|l| l.trim().parse()).collect()
}

#[aoc(day9, part1)]
pub fn part1(numbers: &[usize]) -> usize {
    decode_xmas(numbers, 25)
}

#[aoc(day9, part2)]
pub fn part2(numbers: &[usize]) -> usize {
    let n = decode_xmas(numbers, 25);
    encryption_weakness(numbers, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    #[test]
    fn sample1() {
        let parsed = parse_input_day9(SAMPLE).unwrap();
        assert_eq!(decode_xmas(&parsed, 5), 127);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day9(SAMPLE).unwrap();
        let n = decode_xmas(&parsed, 5);
        assert_eq!(encryption_weakness(&parsed, n), 62);
    }
}
