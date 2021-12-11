use crate::advent::{AdventError, AdventResult, Puzzle};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct SonarSweep(Vec<u32>);

impl TryFrom<&str> for SonarSweep {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        println!("{}", value);
        let res = value
            .lines()
            .map(|x| x.parse::<u32>())
            .collect::<Result<_, _>>()?;
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
    let got = SonarSweep::try_from(input).unwrap();
    assert_eq!(expect, got.0);
}

impl Puzzle for SonarSweep {
    fn first(&self) -> AdventResult<i32> {
        let count = self.0.windows(2).filter(|r| r[0] < r[1]).count();
        Ok(count as i32)
    }
    fn second(&self) -> AdventResult<i32> {
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
