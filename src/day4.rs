use aoc_runner_derive::{aoc, aoc_generator};
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Error {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
    UnknownKey,
    MissingKeys,
}

#[derive(Debug, PartialEq)]
pub enum Height {
    Cm(u8),
    In(u8),
    Unknown(u8),
}

impl Height {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Cm(v) => 150 <= *v && *v <= 193,
            Self::In(v) => 59 <= *v && *v <= 76,
            Self::Unknown(_) => false,
        }
    }
}

impl Default for Height {
    fn default() -> Self {
        Self::Cm(0)
    }
}

impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<u8>() {
            return Ok(Self::Unknown(v));
        };

        match s
            .char_indices()
            .rev()
            .nth(1)
            .map(|(i, _)| (s[..i].parse().expect("Failed to parse height"), &s[i..]))
        {
            Some((v, "in")) => Ok(Self::In(v)),
            Some((v, "cm")) => Ok(Self::Cm(v)),
            _ => return Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Passport {
    /// (Birth Year)
    pub byr: u16,
    /// (Issue Year)
    pub iyr: u16,
    /// (Expiration Year)
    pub eyr: u16,
    /// (Height)
    pub hgt: Height,
    /// (Hair Color)
    pub hcl: String,
    /// (Eye Color)
    pub ecl: String,
    /// (Passport ID)
    pub pid: String,
    /// (Country ID)
    pub cid: Option<u32>,
}

impl Passport {
    pub fn validate(&self) -> Result<(), Error> {
        if self.pid.len() != 9 || self.pid.parse::<u32>().is_err() {
            return Err(Error::Pid);
        }

        if !matches!(
            self.ecl.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ) {
            return Err(Error::Ecl);
        }

        if !self
            .hcl
            .char_indices()
            .all(|c| matches!(c, (0, '#') | (1..=6, 'a'..='f') | (1..=6, '0'..='9')))
        {
            return Err(Error::Hcl);
        }

        if self.byr < 1920 || 2002 < self.byr {
            return Err(Error::Byr);
        }

        if self.iyr < 2010 || 2020 < self.iyr {
            return Err(Error::Iyr);
        }

        if self.eyr < 2020 || 2030 < self.eyr {
            return Err(Error::Eyr);
        }

        if !self.hgt.is_valid() {
            return Err(Error::Hgt);
        }

        Ok(())
    }
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Self::default();
        let mut n_keys = 0;

        for (key, value) in s.split_ascii_whitespace().map(|kv| {
            let kv: Vec<_> = kv.split_terminator(':').collect();
            (kv[0], kv[1])
        }) {
            n_keys += 1;
            match key {
                "byr" => {
                    passport.byr = value.parse().map_err(|_| Error::Byr)?;
                }
                "iyr" => {
                    passport.iyr = value.parse().map_err(|_| Error::Iyr)?;
                }
                "eyr" => {
                    passport.eyr = value.parse().map_err(|_| Error::Eyr)?;
                }
                "hgt" => {
                    passport.hgt = value.parse().map_err(|_| Error::Hgt)?;
                }
                "hcl" => {
                    passport.hcl = value.parse().map_err(|_| Error::Hcl)?;
                }
                "ecl" => {
                    passport.ecl = value.parse().map_err(|_| Error::Ecl)?;
                }
                "pid" => {
                    passport.pid = value.parse().map_err(|_| Error::Pid)?;
                }
                "cid" => {
                    // Ignore CID as a validation parameter
                    n_keys -= 1;
                    passport.cid = Some(value.parse().map_err(|_| Error::Cid)?);
                }
                _ => {
                    return Err(Error::UnknownKey);
                }
            }
        }

        // Require at least 7 fields, `cid` excluded, for a passport to be valid
        if n_keys < 7 {
            Err(Error::MissingKeys)
        } else {
            Ok(passport)
        }
    }
}

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Result<Vec<Passport>, ParseIntError> {
    Ok(input
        .split_terminator("\n\n")
        .filter_map(|p| p.parse().ok())
        .collect())
}

#[aoc(day4, part1)]
pub fn part1(passes: &[Passport]) -> usize {
    passes.len()
}

#[aoc(day4, part2)]
pub fn part2(passes: &[Passport]) -> usize {
    passes
        .iter()
        .filter(|&pass| pass.validate().is_ok())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm

    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929

    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm

    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";

    const INVALID: &str = "eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

    iyr:2019
    hcl:#602927 eyr:1967 hgt:170cm
    ecl:grn pid:012533040 byr:1946

    hcl:dab227 iyr:2012
    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

    hgt:59cm ecl:zzz
    eyr:2038 hcl:74454a iyr:2023
    pid:3556412378 byr:2007";

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f

    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022

    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn parse_hgt() {
        let hgt_cm = "135cm";
        assert_eq!(Height::from_str(hgt_cm), Ok(Height::Cm(135)));

        let hgt_in = "15in";
        assert_eq!(Height::from_str(hgt_in), Ok(Height::In(15)));

        let hgt_in = "156";
        assert_eq!(Height::from_str(hgt_in), Ok(Height::Unknown(156)));
    }

    #[test]
    fn newline_split() {
        let provided = include_str!("../input/2020/day4.txt");
        assert_eq!(provided.split_terminator("\n\n").count(), 279);
    }

    #[test]
    fn input_parser() {
        assert_eq!(
            parse_input_day4(SAMPLE),
            Ok(vec![
                Passport {
                    byr: 1937,
                    iyr: 2017,
                    eyr: 2020,
                    hgt: Height::Cm(183),
                    hcl: "#fffffd".to_owned(),
                    ecl: "gry".to_owned(),
                    pid: "860033327".to_owned(),
                    cid: Some(147)
                },
                Passport {
                    byr: 1931,
                    iyr: 2013,
                    eyr: 2024,
                    hgt: Height::Cm(179),
                    hcl: "#ae17e1".to_owned(),
                    ecl: "brn".to_owned(),
                    pid: "760753108".to_owned(),
                    cid: None
                }
            ])
        );
    }

    #[test]
    fn sample1() {
        let parsed = parse_input_day4(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 2);
    }

    #[test]
    fn hcl_validation() {
        let pass = Passport {
            byr: 1937,
            iyr: 2017,
            eyr: 2020,
            hgt: Height::Cm(183),
            hcl: "#fffffd".to_owned(),
            ecl: "gry".to_owned(),
            pid: "860033327".to_owned(),
            cid: Some(147),
        };

        let fail1 = Passport {
            byr: 1937,
            iyr: 2017,
            eyr: 2020,
            hgt: Height::Cm(183),
            hcl: "fffffd".to_owned(),
            ecl: "gry".to_owned(),
            pid: "860033327".to_owned(),
            cid: Some(147),
        };

        let fail2 = Passport {
            byr: 1937,
            iyr: 2017,
            eyr: 2020,
            hgt: Height::Cm(183),
            hcl: "#ffftfd".to_owned(),
            ecl: "gry".to_owned(),
            pid: "860033327".to_owned(),
            cid: Some(147),
        };


        assert_eq!(pass.validate(), Ok(()));
        assert_eq!(fail1.validate(), Err(Error::Hcl));
        assert_eq!(fail2.validate(), Err(Error::Hcl));
    }

    #[test]
    fn part2_invalid() {
        let parsed = parse_input_day4(INVALID).unwrap();
        assert_eq!(part2(&parsed), 0);
    }

    #[test]
    fn part2_valid() {
        let parsed = parse_input_day4(VALID).unwrap();
        assert_eq!(part2(&parsed), 4);
    }
}
