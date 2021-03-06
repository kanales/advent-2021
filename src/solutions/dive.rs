// Day 2: Dive! https://adventofcode.com/2021/day/2
use crate::advent::{AdventError, AdventResult, Puzzle};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Fwd(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Instruction {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use AdventError::*;
        use Instruction::*;
        let mut parts = value.split_whitespace();
        let left = parts
            .next()
            .ok_or_else(|| ParseError("missing action".to_owned()))?;
        let right = parts
            .next()
            .ok_or_else(|| ParseError("missing value".to_owned()))?;
        let right: i32 = right.parse::<i32>()?;

        Ok(match left {
            "forward" => Fwd(right),
            "up" => Up(right),
            "down" => Down(right),
            x => {
                return Err(ParseError(format!(
                    "Found {} expected one of: 'forward', 'up', 'down'",
                    x
                )))
            }
        })
    }
}

#[derive(Debug)]
pub struct Dive(Vec<Instruction>);

impl FromStr for Dive {
    type Err = AdventError;

    fn from_str(value: &str) -> AdventResult<Self> {
        let insts = value
            .lines()
            .map(Instruction::from_str)
            .collect::<AdventResult<_>>()?;
        Ok(Dive(insts))
    }
}

#[test]
fn parsing() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    use Instruction::*;
    let expect = vec![Fwd(5), Down(5), Fwd(8), Up(3), Down(8), Fwd(2)];
    let dive = Dive::from_str(input).unwrap();
    assert_eq!(dive.0, expect)
}

impl Puzzle for Dive {
    fn second(&self) -> AdventResult<i64> {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for k in &self.0 {
            match k {
                Instruction::Up(v) => aim -= v,
                Instruction::Down(v) => aim += v,
                Instruction::Fwd(v) => {
                    x += v;
                    y += aim * v
                }
            }
        }
        Ok((x * y) as i64)
    }

    fn first(&self) -> AdventResult<i64> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for k in &self.0 {
            match k {
                Instruction::Up(v) => y -= v,
                Instruction::Down(v) => y += v,
                Instruction::Fwd(v) => x += v,
            }
        }
        Ok((x * y) as i64)
    }
}

#[test]
fn first() {
    use Instruction::*;
    let dive = Dive(vec![Fwd(5), Down(5), Fwd(8), Up(3), Down(8), Fwd(2)]);
    assert_eq!(dive.first().unwrap(), 150)
}

#[test]
fn second() {
    use Instruction::*;
    let dive = Dive(vec![Fwd(5), Down(5), Fwd(8), Up(3), Down(8), Fwd(2)]);
    assert_eq!(dive.second().unwrap(), 900)
}
