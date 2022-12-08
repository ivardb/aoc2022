use std::cmp::{max, Ordering};
use std::fs;
use itertools::Itertools;
use crate::day8::Direction::{Down, Left, Right, Up};

#[derive(Copy, Clone, Debug)]
struct Pos {
    row: usize,
    col: usize
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn all_directions() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }
}

impl Pos {
    fn update(self, direction: Direction, row_bound: usize, col_bound: usize) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.row == 0 {
                    None
                } else {
                    Some(Pos {row: self.row - 1, col: self.col})
                }
            },
            Direction::Down => {
                if self.row == row_bound - 1 {
                    None
                } else {
                    Some(Pos {row: self.row + 1, col: self.col})
                }
            },
            Direction::Left => if self.col == 0 {
                None
            } else {
                Some(Pos {row: self.row, col: self.col - 1})
            },
            Direction::Right => {
                if self.col == col_bound - 1 {None} else {
                    Some(Pos {row: self.row, col: self.col + 1})
                }
            },
        }
    }
}

fn viewing_distance(data: &Vec<Vec<u32>>, direction: Direction, start: Pos) -> usize {
    let mut count = 0;
    let height = data[start.row][start.col];
    let mut pos = start;
    while let Some(p) = pos.update(direction, data.len(), data[0].len()) {
        pos = p;
        count += 1;
        if data[pos.row][pos.col] >= height {
            break
        }
    }
    count
}

pub fn day8() {
    let data = fs::read_to_string("data/day8.txt").unwrap().split('\n')
        .map(|l| Vec::from_iter(l.chars().map(|c| c.to_digit(10).unwrap()))).collect_vec();
    let mut visible = Vec::new();
    for v in &data {
        let mut nv = Vec::new();
        for _ in v {
            nv.push(false);
        }
        visible.push(nv);
    }
    for i in 0..data.len() {
        visible[i][0] = true;
        let mut high = data[i][0];
        let mut j = 1;
        while high < 9 && j < data[i].len() {
            if data[i][j] > high {
                high = data[i][j];
                visible[i][j] = true;
            }
            j += 1;
        }
        j = data[i].len() - 2;
        visible[i][j+1] = true;
        high = data[i][j+1];
        while high < 9 && j > 0 {
            if data[i][j] > high {
                high = data[i][j];
                visible[i][j] = true;
            }
            j -= 1;
        }
    }
    for j in 0..data[0].len() {
        visible[0][j] = true;
        let mut high = data[0][j];
        let mut i = 1;
        while high < 9 && i < data.len() {
            if data[i][j] > high {
                high = data[i][j];
                visible[i][j] = true;
            }
            i += 1;
        }
        i = data.len() - 2;
        visible[i+1][j] = true;
        high = data[i+1][j];
        while high < 9 && i > 0 {
            if data[i][j] > high {
                high = data[i][j];
                visible[i][j] = true;
            }
            i -= 1;
        }
    }
    let mut maxi = 0;
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            maxi = max(Direction::all_directions().map(|d| viewing_distance(&data, d, Pos{row, col})).into_iter().product(), maxi);
        }
    }
    println!("Part 1: {}", visible.iter().map(|v| v.iter().filter(|b| **b).count()).sum::<usize>());
    println!("Part 2: {}", maxi)
}