use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

pub fn day3() {
    let sacks = fs::read_to_string("data/day3.txt").unwrap().split_ascii_whitespace().map(String::from).collect_vec();
    let mut prio = 0;
    for sack in &sacks {
        for c in sack[0..sack.len()/2].chars() {
            if sack[sack.len()/2..sack.len()].contains(c) {
                prio += priority(c);
                break
            }
        }
    }
    let mut prio2 = 0;
    let mut iter = sacks.iter().map(|s| HashSet::from_iter(s.chars()));
    while let Some(s) = iter.next() {
        let s2: HashSet<char> = iter.next().unwrap();
        let s3 = iter.next().unwrap();
        let s4 = HashSet::from_iter(s2.intersection(&s3).copied());
        prio2 += priority(*s.intersection(&s4).next().unwrap());
    }
    println!("Part 1: {prio}");
    println!("Part 2: {prio2}")
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}