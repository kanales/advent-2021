use crate::advent::{AdventError, AdventResult, Puzzle};
use crate::parse_error;
use std::convert::TryFrom;

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

impl TryFrom<&str> for BinaryDiagnostic {
    type Error = AdventError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<u32> = value.lines().map(parse_int).collect::<AdventResult<_>>()?;
        if lines.is_empty() {
            return Err(AdventError::EofError);
        }
        let digits = value.lines().next().unwrap().len();
        Ok(BinaryDiagnostic {
            values: lines,
            digits,
        })
    }
}

impl<'a> Puzzle<'a> for BinaryDiagnostic {
    fn first(&self) -> AdventResult<i32> {
        Ok((self.epsilon() * self.gamma()) as i32)
    }
    fn second(&self) -> AdventResult<i32> {
        Ok((self.co2_rating() * self.oxygen_rating()) as i32)
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

    fn oxygen_rating(&self) -> i32 {
        let gamma = self.gamma();
        let mut i = self.digits - 1;
        let mut mask = 1 << i;
        let mut sel = gamma & mask;
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

            if balance >= 0 {
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

    fn co2_rating(&self) -> i32 {
        let epsilon = self.epsilon();
        let mut i = self.digits - 1;
        let mut mask = 1 << i;
        let mut sel = epsilon & mask;
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

            if balance < 0 {
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
        BinaryDiagnostic::try_from(input).unwrap()
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