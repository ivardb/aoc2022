use std::collections::HashSet;
use std::fs;
use crate::day10::Instr::{AddX, NoOp};

enum Instr {
    NoOp,
    AddX(i32),
}

pub fn day10() {
    let data = fs::read_to_string("data/day10.txt").unwrap();
    let mut x : i32 = 1;
    let mut score = 0;
    let mut delay = 0;
    let mut iter = data.split_ascii_whitespace();
    let iv = iter.next().unwrap();
    let mut instr = match iv {
        "noop" => {
            delay = 1;
            NoOp
        },
        _ => {
            delay = 2;
            AddX(iter.next().unwrap().parse().unwrap())
        }
    };
    let counting_rounds : HashSet<_> = (20..=220).step_by(40).collect();
    for round in 1.. {
        let draw_pos = (round - 1) % 40;
        if draw_pos == 0 {
            println!()
        }
        if x.abs_diff(draw_pos) <= 1 {
            print!("█");
        } else {
            print!(" ")
        }
        if counting_rounds.contains(&round) {
            score += round * x;
        }
        delay -= 1;
        if delay == 0 {
            match instr {
                NoOp => {}
                AddX(dx) => {x += dx}
            }
            if let Some(iv) = iter.next() {
                instr = match iv {
                    "noop" => {
                        delay = 1;
                        NoOp
                    },
                    _ => {
                        delay = 2;
                        AddX(iter.next().unwrap().parse().unwrap())
                    }
                }
            } else {break}
        }
    }
    println!("\nPart 1: {score}");
}