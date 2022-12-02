use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use crate::day2::RPS::{Paper, Rock, Scissors};

#[derive(Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(())
        }
    }
}

impl RPS {
    fn play(&self, other: &RPS) -> i32 {
        self.score() + match (self, other) {
            (x, y) if x == y => 3,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Scissors, Paper) => 6,
            _ => 0
        }
    }

    fn score(&self) -> i32 {
        match self {
            Rock => {1}
            Paper => {2}
            Scissors => {3}
        }
    }

    fn losing(&self) -> RPS {
        match self {
            Rock => {Scissors}
            Paper => {Rock}
            Scissors => {Paper}
        }
    }

    fn winning(&self) -> RPS {
        match self {
            Rock => {Paper}
            Paper => {Scissors}
            Scissors => {Rock}
        }
    }
}

pub fn day2() {
    let plays = fs::read_to_string("data/day2.txt").unwrap().split("\n")
        .map(|s| {
            let (x,y) = s.split_once(' ').unwrap();
            (String::from(x), String::from(y))
        }).collect_vec();
    let mut score = 0;
    for (opp, play) in &plays {
        score += play.parse::<RPS>().unwrap().play(&opp.parse().unwrap())
    }
    let mut score2 = 0;
    for (opp, end) in plays {
        let popp = opp.parse::<RPS>().unwrap();
        match end.as_str() {
            "X" => {score2 += popp.losing().score()},
            "Y" => {score2 += 3 + popp.score()},
            _ => {score2 += 6 + popp.winning().score()}
        }
    }
    println!("Part 1: {score}");
    println!("Part 2: {score2}");
}
