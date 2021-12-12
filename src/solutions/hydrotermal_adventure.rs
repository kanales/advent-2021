use crate::advent::{AdventError, AdventResult, Puzzle};
use std::str::FromStr;
use std::{fmt, vec};

#[derive(Debug)]
struct Segment {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Segment {
    fn ortho(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn iter(&self) -> IterSegment {
        use std::cmp::Ordering;
        let dx = match self.x1.cmp(&self.x2) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };

        let dy = match self.y1.cmp(&self.y2) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        IterSegment {
            x: self.x1 as usize,
            y: self.y1 as usize,
            dx,
            dy,
            goalx: self.x2 as usize,
            goaly: self.y2 as usize,
            done: false,
        }
    }
}

pub struct HydrothermalVenture {
    segments: Vec<Segment>,
}

impl HydrothermalVenture {
    pub fn size(&self) -> (usize, usize) {
        let mut cols = 0;
        let mut rows = 0;
        for seg in &self.segments {
            cols = max(max(cols, seg.x1 + 1), seg.x2 + 1);
            rows = max(max(rows, seg.y1 + 1), seg.y2 + 1);
        }
        (cols as usize, rows as usize)
    }
}

fn debug_board(board: &[u32], rows: usize, cols: usize) {
    for y in 0..rows {
        for x in 0..cols {
            print!("{} ", &board[x + cols * y])
        }
        println!();
    }
}

use std::cmp::max;
impl Puzzle for HydrothermalVenture {
    fn first(&self) -> AdventResult<i32> {
        let (cols, rows) = self.size();
        let mut orthos = vec![0u32; cols * rows];
        for seg in &self.segments {
            if seg.ortho() {
                for (x, y) in seg.iter() {
                    println!("{} | {} {}", seg, x, y);
                    let idx = x + cols * y;
                    orthos[idx] += 1;
                }
            }
        }
        debug_board(&orthos, rows, cols);

        let count = orthos.iter().filter(|&x| *x > 1).count();
        Ok(count as i32)
    }

    fn second(&self) -> AdventResult<i32> {
        let (cols, rows) = self.size();
        let mut orthos = vec![0u32; cols * rows];
        for seg in &self.segments {
            for (x, y) in seg.iter() {
                let idx = x + cols * y;
                orthos[idx] += 1;
            }
        }

        let count = orthos.iter().filter(|&x| *x > 1).count();
        Ok(count as i32)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::advent::Puzzle;

    use super::HydrothermalVenture;
    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn first() {
        let hv = HydrothermalVenture::from_str(INPUT).unwrap();
        let got = hv.first().unwrap();
        assert_eq!(got, 5)
    }

    #[test]
    fn second() {
        let hv = HydrothermalVenture::from_str(INPUT).unwrap();
        let got = hv.second().unwrap();
        assert_eq!(got, 12)
    }
}

//
impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)?;
        Ok(())
    }
}

impl FromStr for Segment {
    type Err = AdventError;
    fn from_str(line: &str) -> Result<Self, AdventError> {
        let line: Vec<u32> = line
            .split(" -> ")
            .flat_map(|x| x.split(','))
            .map(u32::from_str)
            .collect::<Result<_, _>>()?;
        if line.len() != 4 {
            return Err(AdventError::EofError);
        }

        Ok(Segment {
            x1: line[0],
            y1: line[1],
            x2: line[2],
            y2: line[3],
        })
    }
}

impl FromStr for HydrothermalVenture {
    type Err = AdventError;
    fn from_str(x: &str) -> Result<Self, AdventError> {
        let segments: Vec<_> = x.lines().map(Segment::from_str).collect::<Result<_, _>>()?;

        Ok(HydrothermalVenture { segments })
    }
}

#[derive(Debug)]
struct IterSegment {
    dx: isize,
    dy: isize,
    x: usize,
    y: usize,
    goalx: usize,
    goaly: usize,
    done: bool,
}

impl Iterator for IterSegment {
    type Item = (usize, usize); // idx

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let (x, y) = (self.x, self.y);

        if x == self.goalx && y == self.goaly {
            self.done = true;
        } else {
            self.x = (self.x as isize + self.dx) as usize;
            self.y = (self.y as isize + self.dy) as usize;
        }

        Some((x, y))
    }
}
