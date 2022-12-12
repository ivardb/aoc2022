use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use crate::Util::{Direction, Pos};

fn load_data() -> (Pos, Pos, Vec<Vec<i32>>, HashSet<Pos>) {
    let data = fs::read_to_string("data/day12.txt").unwrap();
    let mut aset = HashSet::new();
    let mut grid = Vec::new();
    let mut start = Pos {row: 0, col: 0};
    let mut target = Pos {row: 0, col: 0};
    for (row, line) in data.split_ascii_whitespace().enumerate() {
        let mut new_row = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                new_row.push(0);
                start = Pos {row, col};
                aset.insert(start);
            } else if c == 'E' {
                new_row.push('z' as i32 - 'a' as i32);
                target = Pos {row, col }
            } else {
                if c == 'a' {
                    aset.insert(Pos {row, col});
                }
                new_row.push(c as i32 - 'a' as i32)
            }
        }
        grid.push(new_row)
    }
    return (start, target, grid, aset)
}

struct Node {
    cost: i32,
    heuristic: i32,
    pos: Pos,
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

fn shortest_path(mut queue: BinaryHeap<Node>, t: Pos, grid: &Vec<Vec<i32>>) -> i32 {
    let target_height = grid[t.row][t.col];
    let mut visited = HashSet::new();
    while let Some(n) = queue.pop() {
        if n.pos == t {
            return n.cost
        }
        if !visited.insert(n.pos) {
            continue
        }
        let height = grid[n.pos.row][n.pos.col];
        for d in Direction::all_directions() {
            if let Some(new_pos) = n.pos.update(d, grid.len(), grid[0].len()) {
                let new_height = grid[new_pos.row][new_pos.col];
                if !visited.contains(&new_pos) && new_height - height <= 1 {
                    let new_node = Node { pos: new_pos, cost: n.cost + 1, heuristic: max(new_pos.dist(t) as i32, (target_height - new_height)) };
                    queue.push(new_node)
                }
            }
        }
    }
    -1
}

pub fn day12() {
    let (s, t, grid, aset) = load_data();
    let mut queue = BinaryHeap::new();
    queue.push(Node {pos: s, cost: 0, heuristic: max(s.dist(t) as i32, (grid[t.row][t.col] - grid[s.row][s.col]))});
    println!("Part 1: {}", shortest_path(queue, t, &grid));
    let mut queue = BinaryHeap::new();
    queue.extend(aset.iter().map(|p| Node {pos: *p, cost: 0, heuristic: grid[t.row][t.col] - grid[s.row][s.col]}));
    println!("Part 2: {}", shortest_path(queue, t, &grid))
}