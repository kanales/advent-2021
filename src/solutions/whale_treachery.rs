use std::str::FromStr;

use crate::advent::{AdventError, AdventResult, Puzzle};

#[derive(Debug)]
pub struct CrabSwarm(Vec<i64>);

fn median<'a, F>(it: F) -> i64
where
    F: Iterator<Item = &'a i64>,
{
    let mut col: Vec<_> = it.into_iter().collect();
    let l = col.len();
    col.sort();
    if l % 2 == 1 {
        *col[l / 2]
    } else {
        (col[(l - 1) / 2] + col[(l + 1) / 2]) / 2
    }
}

fn gauss(x: i64) -> i64 {
    (x * x + x.abs()) / 2
}
impl Puzzle for CrabSwarm {
    fn first(&self) -> AdventResult<i64> {
        let m = median(self.0.iter());
        let diffs = self.0.iter().map(|&x| (x - m).abs()).sum();
        Ok(diffs)
    }

    fn second(&self) -> AdventResult<i64> {
        let top = self.0.iter().max().unwrap();
        let h = |y: &i64| self.0.iter().map(|x| gauss(y - x)).sum::<i64>();
        let x = (0..=*top).min_by_key(h).unwrap();
        Ok(h(&x))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::advent::Puzzle;

    use super::CrabSwarm;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn first() {
        let crab = CrabSwarm::from_str(INPUT).unwrap();
        assert_eq!(crab.first(), Ok(37))
    }

    #[test]
    fn secodn() {
        let crab = CrabSwarm::from_str(INPUT).unwrap();

        assert_eq!(crab.second(), Ok(168))
    }
}

//
impl FromStr for CrabSwarm {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CrabSwarm(
            s.trim()
                .split(',')
                .map(i64::from_str)
                .collect::<Result<_, _>>()?,
        ))
    }
}
