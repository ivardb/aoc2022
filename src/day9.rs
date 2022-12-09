use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn update(self, dir: &str) -> Self {
        match dir {
            "U" => Point { x: self.x, y: self.y - 1 },
            "D" => Point { x: self.x, y: self.y + 1 },
            "L" => Point { x: self.x - 1, y: self.y },
            _ => Point { x: self.x + 1, y: self.y }
        }
    }
}

fn fix_tail(head: Point, tail: Point) -> Point {
    if head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1 {
        tail
    } else if head.x.abs_diff(tail.x) == 2 && head.y.abs_diff(tail.y) == 2 {
        Point { x: (head.x+tail.x)/2, y: (head.y+tail.y)/2 }
    } else if head.x.abs_diff(tail.x) == 2 {
        Point {x: (head.x+tail.x)/2, y: head.y }
    } else {
        Point { y: (head.y + tail.y) / 2, x: head.x }
    }
}

fn execute(data: String, num_points: usize) -> usize {
    let mut points= Vec::new();
    for _ in 0..num_points {
        points.push(Point {x: 0, y: 0})
    }
    let mut visited = HashSet::new();
    visited.insert(points[num_points-1]);
    for (dir, num) in data.split_ascii_whitespace().tuples() {
        for _ in 0..num.parse::<i32>().unwrap() {
            points[0] = points[0].update(dir);
            for i in 1..num_points {
                points[i] = fix_tail(points[i-1], points[i])
            }
            visited.insert(points[num_points-1]);
        }
    }
    visited.len()
}

pub fn day9() {
    let data = fs::read_to_string("data/day9.txt").unwrap();
    println!("Part 1: {}", execute(data.clone(), 2));
    println!("Part 2: {}", execute(data.clone(), 10));
}