use std::collections::VecDeque;
use std::fs;
use std::ops::{Add, Mul};
use itertools::Itertools;

struct Monkey<F> where F: Fn(u64) -> u64 {
    items: VecDeque<u64>,
    divisor: u64,
    if_true: usize,
    if_false: usize,
    operation: F
}

fn load_data() -> Vec<Monkey<impl Fn(u64) -> u64>> {
    let data = fs::read_to_string("data/day11.txt").unwrap();
    data.split("\n\n").map(|s| {
        let lines = s.split('\n').skip(1).collect_vec();
        let items = lines[0].split([':', ',']).skip(1).map(|n| n.trim().parse().unwrap()).collect();
        let (_, form) = lines[1].split_once(" = ").unwrap();
        let f = form.split_ascii_whitespace().map(String::from).collect_vec();
        let (l1, l2) = if f[0] == "old" {(1, 0)} else {(0, f[0].parse::<u64>().unwrap())};
        let (r1, r2) = if f[2] == "old" {(1, 0)} else {(0, f[2].parse::<u64>().unwrap())};
        let op = f[1].chars().next().unwrap();
        let operation = move |x: u64| (|l1, l2, r1, r2 : u64| {
            if op == '*' {
                (l1 * x + l2) * (r1 * x + r2)
            } else {
                (l1 * x + l2) + (r1 * x + r2)
            }
        })(l1, l2, r1, r2);
        let divisor = lines[2].split_ascii_whitespace().collect_vec()[3].parse().unwrap();
        let if_true = lines[3].split_ascii_whitespace().collect_vec()[5].parse().unwrap();
        let if_false = lines[4].split_ascii_whitespace().collect_vec()[5].parse().unwrap();
        Monkey {items, operation, divisor, if_true, if_false}
    }).collect_vec()
}

fn part1(mut monkeys: Vec<Monkey<impl Fn(u64) -> u64>>) {
    let mut counts = Vec::new();
    for _ in &monkeys {
        counts.push(0);
    }
    for _ in 0..20 {
        for mindex in 0..monkeys.len() {
            while let Some(item) = monkeys[mindex].items.pop_front() {
                counts[mindex] += 1;
                let new_level = (monkeys[mindex].operation)(item) / 3;
                let target = if new_level % monkeys[mindex].divisor == 0 { monkeys[mindex].if_true } else { monkeys[mindex].if_false };
                monkeys[target].items.push_back(new_level);
            }
        }
    }
    println!("Part 1: {}", counts.iter().sorted_by(|c1, c2| c2.cmp(c1)).take(2).product::<i32>());
}

fn part2(mut monkeys: Vec<Monkey<impl Fn(u64) -> u64>>) {
    let total_div : u64 = monkeys.iter().map(|m| m.divisor).product();
    let mut counts = Vec::new();
    for _ in &monkeys {
        counts.push(0);
    }
    for _ in 0..10000 {
        for mindex in 0..monkeys.len() {
            while let Some(item) = monkeys[mindex].items.pop_front() {
                counts[mindex] += 1;
                let new_level = (monkeys[mindex].operation)(item) % total_div;
                let target = if new_level % monkeys[mindex].divisor == 0 { monkeys[mindex].if_true } else { monkeys[mindex].if_false };
                monkeys[target].items.push_back(new_level);
            }
        }
    }
    println!("Part 1: {}", counts.iter().sorted_by(|c1, c2| c2.cmp(c1)).take(2).product::<u64>());
}

pub fn day11() {
    let data = load_data();
    part1(data);
    let data = load_data();
    part2(data);
}