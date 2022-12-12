use crate::Util::Direction::{Down, Left, Right, Up};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn all_directions() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }
}

impl Pos {
    pub fn update(self, direction: Direction, row_bound: usize, col_bound: usize) -> Option<Self> {
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

    pub fn dist(&self, other: Pos) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}