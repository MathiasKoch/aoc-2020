use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, num::ParseIntError};

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input.trim().lines().map(|l| l.trim().parse()).collect()
}

#[aoc(day10, part1)]
pub fn part1(adaptors: &[u8]) -> usize {
    let mut sorted_adaptors = adaptors.to_owned();
    sorted_adaptors.sort();

    println!("Found all my adaptors: {:?}!", sorted_adaptors);

    let mut reached_jolts: u8 = 0;
    let mut diff_cnt: HashMap<u8, usize> = HashMap::new();
    for adaptor in sorted_adaptors {
        print!("Testing adaptor {:?}... ", adaptor);
        if adaptor - reached_jolts <= 3 {
            println!("Success {:?}", adaptor - reached_jolts);

            *diff_cnt.entry(adaptor - reached_jolts).or_insert(0) += 1;
            reached_jolts = adaptor;
        }
    }

    // Last difference of 3 that my device auto-tollerates
    *diff_cnt.entry(3).or_insert(0) += 1;

    diff_cnt.get(&1).unwrap_or(&0) * diff_cnt.get(&3).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    const SAMPLE2: &str = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    #[test]
    fn sample1() {
        let parsed = parse_input_day10(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 7 * 5);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day10(SAMPLE2).unwrap();
        assert_eq!(part1(&parsed), 22 * 10);
    }
}
