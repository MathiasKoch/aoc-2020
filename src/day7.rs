use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, str::FromStr};
use std::{collections::VecDeque, num::ParseIntError};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char as ch, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{Finish, IResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BagColor(String, String);

impl BagColor {
    pub fn new(description: &str, color: &str) -> Self {
        Self(description.to_owned(), color.to_owned())
    }
}

fn color(input: &str) -> IResult<&str, BagColor> {
    let (input, (desc, _, color)) = tuple((alpha1, ch(' '), alpha1))(input)?;
    Ok((input, BagColor::new(desc, color)))
}

fn content_item(input: &str) -> IResult<&str, (BagColor, usize)> {
    let single = map(tuple((tag("1 "), color, tag(" bag"))), |(_, color, _)| {
        (color, 1)
    });
    let multi = map(
        tuple((digit1::<&str, _>, ch(' '), color, tag(" bags"))),
        |(cnt, _, color, _)| {
            let cnt: usize = cnt.parse().unwrap();
            (color, cnt)
        },
    );

    alt((single, multi))(input)
}

fn contents(input: &str) -> IResult<&str, HashMap<BagColor, usize>> {
    let non_empty = map(separated_list1(tag(", "), content_item), |data| {
        data.into_iter().collect()
    });
    let empty = map(tag("no other bags"), |_| HashMap::new());

    alt((non_empty, empty))(input)
}

#[derive(Debug, PartialEq)]
pub struct Bag {
    pub color: BagColor,
    pub contents: HashMap<BagColor, usize>,
}

impl Bag {
    pub fn parse(input: &str) -> IResult<&str, Bag> {
        let (input, (color, _, contents, _)) =
            tuple((color, tag(" bags contain "), contents, ch('.')))(input)?;
        Ok((input, Bag { color, contents }))
    }
}

impl FromStr for Bag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, entry) = Self::parse(s.trim()).finish().map_err(drop)?;

        Ok(entry)
    }
}

#[derive(Debug, Default, Clone)]
struct BagWithIndirect {
    contents: HashMap<BagColor, usize>,
}

impl BagWithIndirect {
    pub fn contains(&self, bag: &BagColor) -> bool {
        self.contents.contains_key(bag)
    }

    pub fn total_bags(&self) -> usize {
        self.contents.iter().map(|(_, cnt)| *cnt).sum()
    }
}

fn build_mappings<'a>(bags: impl Iterator<Item = &'a Bag>) -> HashMap<BagColor, BagWithIndirect> {
    let mut direct_only: HashMap<_, _> = bags
        .map(|bag| (bag.color.clone(), bag.contents.clone()))
        .collect();

    let mut queue: VecDeque<_> = direct_only.iter().map(|(color, _)| color.clone()).collect();

    let mut with_indirect: HashMap<BagColor, BagWithIndirect> = HashMap::new();

    while let Some(bag) = queue.pop_front() {
        if with_indirect.get(&bag).is_some() {
            continue;
        }

        let directs = direct_only.entry(bag.clone()).or_default();
        if directs
            .keys()
            .all(|color| with_indirect.get(color).is_some())
        {
            let mut contents = directs.clone();

            for (child, child_cnt) in directs.iter() {
                for (color, cnt) in with_indirect.get(child).unwrap().contents.iter() {
                    *contents.entry(color.clone()).or_default() += cnt * child_cnt;
                }
            }

            with_indirect.insert(bag, BagWithIndirect { contents });
        } else {
            queue.push_back(bag)
        }
    }

    with_indirect
}

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> Result<Vec<Bag>, ParseIntError> {
    Ok(input.lines().map(|l| l.parse().unwrap()).collect())
}

#[aoc(day7, part1)]
pub fn part1(bags: &[Bag]) -> usize {
    build_mappings(bags.into_iter())
        .values()
        .filter(|bags| bags.contains(&BagColor::new("shiny", "gold")))
        .count()
}

#[aoc(day7, part2)]
pub fn part2(bags: &[Bag]) -> usize {
    build_mappings(bags.into_iter())
        .get(&BagColor::new("shiny", "gold"))
        .expect("No such bag!")
        .total_bags()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";

    const SAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags.";

    #[test]
    fn parse_single_bag() {
        assert_eq!(
            Bag::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Ok(Bag {
                color: BagColor::new("light", "red"),
                contents: vec![
                    (BagColor::new("bright", "white"), 1),
                    (BagColor::new("muted", "yellow"), 2)
                ]
                .iter()
                .cloned()
                .collect()
            })
        );
    }

    #[test]
    fn parse_sample() {
        let parsed = parse_input_day7(SAMPLE).unwrap();
        assert_eq!(parsed.len(), 9);
        assert_eq!(
            parsed[0],
            Bag {
                color: BagColor::new("light", "red"),
                contents: vec![
                    (BagColor::new("bright", "white"), 1),
                    (BagColor::new("muted", "yellow"), 2)
                ]
                .iter()
                .cloned()
                .collect()
            }
        );
    }

    #[test]
    fn sample1() {
        let parsed = parse_input_day7(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 4);
    }

    #[test]
    fn sample1_part2() {
        let parsed = parse_input_day7(SAMPLE).unwrap();
        assert_eq!(part2(&parsed), 32);
    }

    #[test]
    fn sample2_part2() {
        let parsed = parse_input_day7(SAMPLE2).unwrap();
        assert_eq!(part2(&parsed), 126);
    }
}
