use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, num::ParseIntError};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
pub enum Instruction {
    Jump(isize),
    Acc(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().splitn(2, ' ');

        let inst = iter.next().expect("No instruction!");
        let cnt = iter.last().expect("No count!").parse().map_err(drop)?;

        match inst {
            "nop" => Ok(Self::Nop(cnt)),
            "jmp" => Ok(Self::Jump(cnt)),
            "acc" => Ok(Self::Acc(cnt)),
            _ => Err(()),
        }
    }
}

pub struct Patcher {
    patch_nr: usize
}

impl Patcher {
    pub fn new() -> Self {
        Self {
            patch_nr: 0
        }
    }

    pub fn patch_program(
        &mut self,
        program: &HashMap<usize, Instruction>,
    ) -> Option<HashMap<usize, Instruction>> {
        let mut patched_program = program.clone();

        loop {
            if let Some(i) = patched_program.get_mut(&self.patch_nr){
                self.patch_nr += 1;
                match i {
                    Instruction::Nop(v) => *i = Instruction::Jump(*v),
                    Instruction::Jump(v) => *i = Instruction::Nop(*v),
                    _ => {
                        continue;
                    }
                }
                break
            } else {
                return None;
            }
        }

        println!("Patched a new program! {:?}", self.patch_nr);
        Some(patched_program)
    }
}

pub struct Execution {
    accumulator: isize,
    next_line: usize,
    lines_executed: HashSet<usize>,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            next_line: 0,
            lines_executed: HashSet::new(),
        }
    }

    pub fn run(&mut self, program: &HashMap<usize, Instruction>) -> bool {
        loop {
            if let Some(inst) = program.get(&self.next_line) {
                if !self.step(inst) {
                    return false;
                }
            } else {
                return true;
            }
        }
    }

    // pub fn apply_patch(&mut self, inst: &mut Instruction) -> bool {
    //     if self.apply_patches == 0 {
    //         return false;
    //     }

    //     let newinst = match inst {
    //         Instruction::Jump(v) => Instruction::Nop(*v),
    //         Instruction::Acc(v) => Instruction::Acc(*v),
    //         Instruction::Nop(v) => Instruction::Jump(*v)
    //     };
    //     println!("Patched {:?} to {:?}", inst, newinst);
    //     *inst = newinst;
    //     self.apply_patches -= 1;

    //     true
    // }

    pub fn step(&mut self, inst: &Instruction) -> bool {
        println!("Stepping instruction: {:?} - {:?}", self.next_line, inst);

        let newline = match inst {
            Instruction::Jump(cnt) => {
                assert!(self.next_line as isize + cnt >= 0);
                let newline: usize = (self.next_line as isize + cnt) as usize;
                if self.lines_executed.contains(&newline) {
                    return false;
                }
                newline
            }
            Instruction::Acc(cnt) => {
                self.accumulator += cnt;
                self.next_line + 1
            }
            Instruction::Nop(_) => self.next_line + 1,
        };

        self.lines_executed.insert(self.next_line);
        self.next_line = newline;

        true
    }

    pub fn accumulator_value(&self) -> isize {
        self.accumulator
    }
}

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> Result<HashMap<usize, Instruction>, ParseIntError> {
    Ok(input
        .lines()
        .enumerate()
        .map(|(l, s)| (l, s.parse().unwrap()))
        .collect())
}

#[aoc(day8, part1)]
pub fn part1(program: &HashMap<usize, Instruction>) -> isize {
    let mut executor = Execution::new();
    let success = executor.run(program);

    println!("Program terminated with success: {:?}", success);
    executor.accumulator_value()
}

#[aoc(day8, part2)]
pub fn part2(program: &HashMap<usize, Instruction>) -> isize {
    let mut patcher = Patcher::new();
    loop {
        let mut executor = Execution::new();
        if let Some(ref patched_program) = patcher.patch_program(program) {
            if executor.run(patched_program) {
                println!("Program terminated with success!");
                break executor.accumulator_value();
            }
        } else {
            panic!("Failed to find a valid patch!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";

    #[test]
    fn sample1() {
        let parsed = parse_input_day8(SAMPLE).unwrap();
        assert_eq!(part1(&parsed), 5);
    }

    #[test]
    fn sample2() {
        let parsed = parse_input_day8(SAMPLE).unwrap();
        assert_eq!(part2(&parsed), 8);
    }
}
