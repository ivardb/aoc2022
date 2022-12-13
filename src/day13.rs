use std::cmp::{min, Ordering};
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use itertools::Itertools;
use crate::day13::Item::{List, Num};

#[derive(Clone)]
enum Item {
    Num(u32),
    List(Vec<Item>)
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Num(i) => {write!(f, "{}", i)}
            List(v) => {
                write!(f, "[");
                for i in v {
                    write!(f, "{},", i);
                }
                write!(f, "]")
            }
        }
    }
}

impl Eq for Item {}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Num(i), Num(j)) => {i.cmp(j)},
            (List(v1), List(v2)) => {
                for i in 0..min(v1.len(), v2.len()) {
                    let ord = v1[i].cmp(&v2[i]);
                    if ord != Ordering::Equal {
                        return ord
                    }
                }
                v1.len().cmp(&v2.len())
            }
            (n, List(v)) => {
                if v.len() == 0 {
                    Ordering::Greater
                } else {
                    n.cmp(&v[0]).then(1.cmp(&v.len()))
                }
            },
            (List(v), n) => {
                if v.len() == 0 {
                    Ordering::Less
                } else {
                    v[0].cmp(n).then(v.len().cmp(&1))
                }
            }
        }
    }
}

fn parse_item(s : &mut Peekable<Chars>) -> Item {
    if *s.peek().unwrap() == '[' {
        parse_list(s)
    } else {
        let mut num = 0;
        while *s.peek().unwrap() != ']' && *s.peek().unwrap() != ',' {
            num *= 10;
            num += s.next().unwrap().to_digit(10).unwrap();
        }
        Num(num)
    }
}

fn parse_list(s: &mut Peekable<Chars>) -> Item {
    s.next();
    let mut items = Vec::new();
    while let Some(c) = s.peek() {
        match c {
            ']' => {s.next();break},
            ',' => {s.next(); items.push(parse_item(s))},
            _ => items.push(parse_item(s))
        }
    }
    List(items)
}

pub fn day13() {
    let data = fs::read_to_string("data/day13.txt").unwrap();
    let pairs: Vec<(Item, Item)> = data.split("\n\n").map(|s| {
        let (l, r) = s.split_once('\n').unwrap();
        (parse_list(&mut l.chars().peekable()), parse_list(&mut r.chars().peekable()))
    }).collect_vec();
    println!("Part 1: {:?}", pairs.iter()
        .enumerate()
        .map(|(i, (l, r))| if l.cmp(r).is_lt() {i + 1} else {0}).sum::<usize>());
    let mut packets = Vec::new();
    for (l, r) in pairs {
        packets.push(l);
        packets.push(r)
    }
    let divider1 = List(vec![List(vec![Num(2)])]);
    let divider2 = List(vec![List(vec![Num(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let p1 = packets.binary_search(&divider1).unwrap();
    let p2 = packets.binary_search(&divider2).unwrap();
    println!("Part 2: {}", (1 + p1) * (1 + p2))
}