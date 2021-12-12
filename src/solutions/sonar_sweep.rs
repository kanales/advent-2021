use std::str::FromStr;

use crate::advent::{AdventError, AdventResult, Puzzle};

#[derive(Debug)]
pub struct SonarSweep(Vec<u32>);

impl FromStr for SonarSweep {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        println!("{}", value);
        let res = value.lines().map(u32::from_str).collect::<Result<_, _>>()?;
        Ok(SonarSweep(res))
    }
}

#[test]
fn parse() {
    let input = "199
200
208
210
200
207
240
269
260
263";
    let expect = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let got = SonarSweep::from_str(input).unwrap();
    assert_eq!(expect, got.0);
}

impl Puzzle for SonarSweep {
    fn first(&self) -> AdventResult<i64> {
        let count = self.0.windows(2).filter(|r| r[0] < r[1]).count();
        Ok(count as i64)
    }
    fn second(&self) -> AdventResult<i64> {
        let window_sums: Vec<_> = self.0.windows(3).map(|r| r[0] + r[1] + r[2]).collect();
        println!("{:?}", window_sums);
        Ok(window_sums
            .windows(2)
            .map(|r| if r[0] < r[1] { 1 } else { 0 })
            .sum())
    }
}

#[test]
fn first() {
    let ss = SonarSweep(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    assert_eq!(ss.first().unwrap(), 7);
}

#[test]
fn second() {
    let ss = SonarSweep(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
    assert_eq!(ss.second().unwrap(), 5);
}
