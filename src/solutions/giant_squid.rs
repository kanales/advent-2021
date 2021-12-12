use crate::advent::{AdventError, AdventResult, Puzzle};
use std::{convert::TryInto, str::FromStr};

#[derive(Debug)]
struct Board {
    /* 5x5 board */
    values: [u32; 5 * 5],
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            let row = (0..5)
                .map(|j| format!("{}", self[(i, j)]))
                .collect::<Vec<String>>()
                .join("\t");
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl Board {
    fn sum_unmarked(&self, vals: &[u32]) -> u32 {
        let mut acc = 0;
        for i in 0..(5 * 5) {
            let v = self.values[i];
            if !vals.contains(&v) {
                acc += v
            }
        }
        acc
    }

    fn bingo(&self, vals: &[u32]) -> Option<u32> {
        let last = vals.last()?;

        /* Check rows */
        for row in 0..5 {
            if (0..5).all(|col| vals.contains(&self[(row, col)])) {
                return Some(*last);
            }
        }

        /* Check cols */
        for col in 0..5 {
            if (0..5).all(|row| vals.contains(&self[(row, col)])) {
                return Some(*last);
            }
        }

        None
    }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = u32;

    fn index(&self, (k, v): (usize, usize)) -> &Self::Output {
        &self.values[k * 5 + v]
    }
}

impl FromStr for Board {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        /*
        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
         */
        let values: Vec<_> = value
            .split_whitespace()
            .map(|v| v.parse::<u32>())
            .collect::<Result<_, _>>()?;
        Ok(Board {
            values: values
                .as_slice()
                .try_into()
                .map_err(|_| AdventError::ParseError("Missing values for board".to_string()))?,
        })
    }
}

#[derive(Debug)]
pub struct GiantSquid {
    inputs: Vec<u32>,
    boards: Vec<Board>,
}

impl FromStr for GiantSquid {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split("\n\n");
        let inputs: Vec<u32> = parts
            .next()
            .ok_or(AdventError::EofError)?
            .split(',')
            .map(u32::from_str)
            .collect::<Result<_, _>>()?;

        let boards: Vec<_> = parts.map(Board::from_str).collect::<AdventResult<_>>()?;
        Ok(GiantSquid { inputs, boards })
    }
}

impl Puzzle for GiantSquid {
    fn first(&self) -> AdventResult<i64> {
        for i in 0..self.inputs.len() {
            for b in &self.boards {
                let vals = &self.inputs[0..i];
                if let Some(r) = b.bingo(vals) {
                    return Ok((r * b.sum_unmarked(vals)) as i64);
                }
            }
        }

        Err(AdventError::EofError)
    }
    fn second(&self) -> AdventResult<i64> {
        let mut winners = vec![false; self.boards.len()];
        let mut count = winners.len();

        for i in 0..self.inputs.len() {
            let vals = &self.inputs[0..i];

            for j in 0..winners.len() {
                if winners[j] {
                    continue;
                }

                if let Some(v) = self.boards[j].bingo(vals) {
                    if count == 1 {
                        let res = self.boards[j].sum_unmarked(vals) * v;
                        return Ok(res as i64);
                    }
                    winners[j] = true;
                    count -= 1;
                }
            }
        }

        Err(AdventError::EofError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> GiantSquid {
        let s = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        GiantSquid::from_str(s).unwrap()
    }

    #[test]
    fn first() {
        let expect = 4512;
        let got = input().first().unwrap();
        assert_eq!(expect, got)
    }

    #[test]
    fn second() {
        let expect = 1924;
        let got = input().second().unwrap();
        assert_eq!(expect, got)
    }
}
