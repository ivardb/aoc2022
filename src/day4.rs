use std::fs;
use itertools::Itertools;

pub fn day4() {
    let data : Vec<(u32, u32, u32, u32)> = fs::read_to_string("data/day4.txt").unwrap()
        .split(['\n', ',', '-'])
        .map(|l| l.parse().unwrap())
        .tuples().collect_vec();
    println!("Part 1: {}", data.iter().filter(|(f1, t1, f2, t2)| {
        (f1 >= f2 && t1 <= t2) || (f2 >= f1 && t2 <= t1)
    }).count());
    println!("Part 2: {}", data.iter().filter(|(f1, t1, f2, t2)| {
        f1 <= t2 && f2 <= t1
    }).count());
}