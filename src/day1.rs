use std::fs;
use itertools::Itertools;

pub fn day1() {
    let data = fs::read_to_string("data/day1.txt").unwrap();
    let elfs : Vec<_> = data.split("\n\n").collect();
    let calories = elfs.iter()
        .map(|s| s.split('\n')
            .map(|s| s.parse::<u32>().unwrap())
            .sum::<u32>())
        .sorted_by(|i1, i2| i2.cmp(i1)).collect_vec();
    println!("Part 1: {}", calories[0]);
    println!("Part 2: {}", calories.iter().take(3).sum::<u32>())
}