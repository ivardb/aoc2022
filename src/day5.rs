use std::fs;
use itertools::Itertools;

type Instr = Vec<(usize, usize, usize)>;

fn load_input() -> (Vec<Vec<char>>, Instr) {
    let data = fs::read_to_string("data/day5.txt").unwrap();
    let (start, instr) = data.split_once("\n\n").unwrap();
    let mut start_lines = start.split('\n').map(String::from).collect_vec();
    let max_len = start_lines.iter().map(|s| s.len()).max().unwrap();
    start_lines = start_lines.iter().map(|s| format!("{s}{n:>width$}", n="", width=max_len-s.len())).collect_vec();
    let col_num = (max_len + 1) / 4;
    let mut cols = Vec::new();
    for _ in 0..col_num {
        cols.push(Vec::new());
    }
    for line in start_lines.iter().rev().skip(1) {
        for (i, chunk) in line.chars().collect_vec().chunks(4).enumerate() {
            if chunk[1] != ' ' {
                cols[i].push(chunk[1])
            }
        }
    }
    (cols, instr.split_ascii_whitespace().tuples().map(|(_,b,_,d,_,f)| (b.parse().unwrap(), d.parse().unwrap(), f.parse().unwrap())).collect_vec())
}

fn part1(mut cols: Vec<Vec<char>>, instr: Instr) {
    for (n, s, d) in instr {
        for _ in 0..n {
            let v = cols[s-1].pop().unwrap();
            cols[d-1].push(v);
        }
    }
    print!("Part 1: ");
    for c in cols {
        print!("{}", c.last().unwrap())
    }
    println!();
}

fn part2(mut cols: Vec<Vec<char>>, instr: Vec<(usize, usize, usize)>) {
    for (n, s, d) in instr {
        let l = cols[s-1].len();
        let v = cols[s-1][l-n..l].iter().copied().collect_vec();
        cols[d-1].extend(v.into_iter());
        for i in 1..=n {
            cols[s-1].remove(l-i);
        }
    }
    print!("Part 1: ");
    for c in cols {
        print!("{}", c.last().unwrap())
    }
    println!();
}

pub fn day5() {
    let (cols, instr) = load_input();
    part1(cols, instr);
    let (cols, instr) = load_input();
    part2(cols, instr);
}