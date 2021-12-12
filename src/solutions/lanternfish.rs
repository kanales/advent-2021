use std::str::FromStr;

use crate::advent::{AdventError, Puzzle};

#[derive(Debug)]
pub struct Lanternfish {
    counted: Vec<usize>,
}

impl Lanternfish {
    fn run(&self, days: i64) -> i64 {
        // init
        let mut map = [0; 9];
        for (i, e) in self.counted.iter().enumerate() {
            map[i] = *e as i64;
        }

        for _ in 0..days {
            // tick
            let birthing = map[0];

            for i in 0..8 {
                map[i] = map[i + 1];
            }

            map[8] = birthing;
            map[6] += birthing;
        }

        map.iter().sum::<i64>()
    }
}

impl Puzzle for Lanternfish {
    fn first(&self) -> crate::advent::AdventResult<i64> {
        let res = self.run(80);
        Ok(res as i64)
    }

    fn second(&self) -> crate::advent::AdventResult<i64> {
        let res = self.run(256);
        Ok(res as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::advent::Puzzle;

    use super::Lanternfish;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first() {
        let lf = INPUT.parse::<Lanternfish>().unwrap();
        println!("{:?}", lf.counted);

        let got = lf.first().unwrap();
        assert_eq!(got, 5934)
    }

    #[test]
    fn second() {
        let lf = INPUT.parse::<Lanternfish>().unwrap();
        let got = lf.second().unwrap();
        assert_eq!(got, 26984457539)
    }
}

//
impl FromStr for Lanternfish {
    type Err = AdventError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let fishes: Vec<_> = value
            .trim_end()
            .split(',')
            .map(u8::from_str)
            .collect::<Result<_, _>>()?;

        let mut map = vec![0; 9];
        for fish in fishes {
            map[fish as usize] += 1;
        }
        Ok(Lanternfish { counted: map })
    }
}
