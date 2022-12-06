use std::fs;
use itertools::Itertools;

fn process_data(data: String, length: usize) -> usize {
    let mut buffer: Vec<char> = Vec::with_capacity(length);
    buffer.extend(data[0..length].chars());
    if buffer.iter().all_unique()  {
        return length;
    }
    let mut i = 0;
    for (j, c) in data.chars().enumerate().skip(length) {
        buffer[i] = c;
        if buffer.iter().all_unique() {
            return j + 1;
        }
        i = (i + 1) % length;
    }
    return 0;
}

pub fn day6() {
    let data = fs::read_to_string("data/day6.txt").unwrap();
    println!("Part 1: {}", process_data(data, 4));
    let data = fs::read_to_string("data/day6.txt").unwrap();
    println!("Part 2: {}", process_data(data, 14));
}