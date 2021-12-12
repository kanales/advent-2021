use crate::advent::{AdventError, AdventResult, Puzzle};
use crate::parse_error;
use std::str::FromStr;

#[derive(Debug)]
pub struct BinaryDiagnostic {
    digits: usize,
    values: Vec<u32>,
}

fn parse_int(s: &str) -> AdventResult<u32> {
    // check
    for c in s.chars() {
        if c != '1' && c != '0' {
            return Err(parse_error!("found {}; expected one of ['0', '1']", c));
        }
    }
    let b = s.bytes().fold(0, |a, x| a * 2 + (x - 48) as u32);
    Ok(b)
}

impl FromStr for BinaryDiagnostic {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<u32> = s.lines().map(parse_int).collect::<AdventResult<_>>()?;
        if lines.is_empty() {
            return Err(AdventError::EofError);
        }
        let digits = s.lines().next().unwrap().len();
        Ok(BinaryDiagnostic {
            values: lines,
            digits,
        })
    }
}

impl Puzzle for BinaryDiagnostic {
    fn first(&self) -> AdventResult<i64> {
        Ok((self.epsilon() * self.gamma()) as i64)
    }
    fn second(&self) -> AdventResult<i64> {
        Ok((self.co2_rating() * self.oxygen_rating()) as i64)
    }
}

impl BinaryDiagnostic {
    fn gamma(&self) -> u32 {
        let half = (self.values.len() / 2) as u32;
        self.values
            .iter()
            .fold(vec![0u32; self.digits], |mut counts, n| {
                (0..self.digits).for_each(|i| {
                    let shift = self.digits - 1 - i;
                    counts[i] += (n & (1 << shift)) >> shift;
                });
                counts
            })
            .iter()
            .fold(0u32, |a, x| a * 2 + if *x > half { 1 } else { 0 })
    }

    fn epsilon(&self) -> u32 {
        !self.gamma() & ((1 << self.digits) - 1)
    }

    fn rating<F>(&self, from: u32, criterion: F) -> i32
    where
        F: Fn(i32) -> bool,
    {
        let mut i = self.digits - 1;
        let mut mask = 1 << i;
        let mut sel = from & mask;
        let mut candidates = self.values.clone();
        loop {
            i -= 1;
            let mut balance = 0;
            candidates = candidates
                .into_iter()
                .filter(|c| {
                    if (c & mask) == sel {
                        balance += if c & (1 << i) > 0 { 1 } else { -1 };
                        true
                    } else {
                        false
                    }
                })
                .collect();

            if criterion(balance) {
                sel |= 1 << i
            }
            mask |= 1 << i;
            match (candidates.len(), i) {
                (1, _) => return candidates[0] as i32,
                (_, 0) => return sel as i32,
                _ => {}
            }
        }
    }

    fn oxygen_rating(&self) -> i32 {
        self.rating(self.gamma(), |i| i >= 0)
    }

    fn co2_rating(&self) -> i32 {
        self.rating(self.epsilon(), |i| i < 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> BinaryDiagnostic {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        BinaryDiagnostic::from_str(input).unwrap()
    }

    #[test]
    fn gamma() {
        let expect = 22;
        let got = input().gamma();
        assert_eq!(expect, got);
    }

    #[test]
    fn epsilon() {
        let expect = 9;
        let got = input().epsilon();
        assert_eq!(expect, got);
    }

    #[test]
    fn oxygen() {
        let expect = 23;
        let got = input().oxygen_rating();
        assert_eq!(expect, got);
    }

    #[test]
    fn co2() {
        let expect = 10;
        let got = input().co2_rating();
        assert_eq!(expect, got);
    }

    #[test]
    fn first() {
        let expect = 198;
        let got = input().first().unwrap();
        assert_eq!(expect, got);
    }

    #[test]
    fn second() {
        let expect = 230;
        let got = input().second().unwrap();
        assert_eq!(expect, got);
    }
}
