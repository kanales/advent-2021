use std::{convert::TryFrom, str::FromStr};

use crate::advent::{AdventError, Puzzle};

pub struct Lanternfish {
    fishes: Vec<u8>,
}

impl Puzzle for Lanternfish {
    fn first(&self) -> crate::advent::AdventResult<i32> {
        let steps = 80;
        let mut fishes = self.fishes.clone();
        for _ in 0..steps {
            let curlen = fishes.len();
            for i in 0..curlen {
                if fishes[i] == 0 {
                    fishes.push(8);
                    fishes[i] = 7;
                }
                fishes[i] -= 1;
            }
        }

        Ok(fishes.len() as i32)
    }

    fn second(&self) -> crate::advent::AdventResult<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Lanternfish;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first() {
        let lf = INPUT.parse::<Lanternfish>().unwrap();
    }
}

//
impl FromStr for Lanternfish {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let fishes: Vec<_> = value
            .split(',')
            .map(|v| v.parse::<u8>())
            .collect::<Result<_, _>>()?;
        Ok(Lanternfish { fishes })
    }
}
