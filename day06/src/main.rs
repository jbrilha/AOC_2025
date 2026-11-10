use core::error;
use std::{
    cmp::max,
    env,
    fmt::{self},
    fs::read_to_string,
    ops::{Add, Mul},
    str::FromStr,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
enum Operation {
    Sum,
    Mul,
}

struct Worksheet {
    pub nums: Vec<Vec<usize>>,
    pub ops: Vec<Operation>,
}

impl Worksheet {
    fn get_total_part1(&mut self) -> usize {
        self.nums
            .iter()
            .enumerate()
            .map(|(idx, r)| match self.ops[idx] {
                Operation::Sum => r.iter().copied().reduce(|acc, n| acc.add(n)).unwrap(),
                Operation::Mul => r.iter().copied().reduce(|acc, n| acc.mul(n)).unwrap(),
            })
            .reduce(|acc, e| acc + e)
            .unwrap()
    }

    fn find_fresh_part2(&mut self) -> usize {
        let mut fresh = 0;

        fresh
    }
}

impl fmt::Display for Worksheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.nums.iter().for_each(|r| {
            r.iter().for_each(|n| {
                write!(f, "{n} ").unwrap();
            });
            write!(f, "\n").unwrap();
        });

        self.ops.iter().for_each(|o| {
            write!(f, "{:?}\n", o).unwrap();
        });

        Ok(())
    }
}

fn parse_nums(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|n| usize::from_str(n).unwrap())
        .collect()
}

fn parse_ops(line: &str) -> Vec<Operation> {
    line.split_whitespace()
        .map(|c| match c {
            "+" => Operation::Sum,
            "*" => Operation::Mul,
            _ => panic!(),
        })
        .collect()
}

// totally not cheating: https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose(v: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    (0..v[0].len())
        .map(|i| {
            v.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn read_ranges_ids(filename: &str) -> Worksheet {
    let mut nums = vec![];
    let mut ops = vec![];
    let mut swap = false;

    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let first = l.chars().next().unwrap();
            if !swap && (first.eq(&'*') || first.eq(&'+')) {
                swap = true;
            }

            if !swap {
                nums.push(parse_nums(l))
            } else {
                ops = parse_ops(l);
            }
        })
        .count();

    Worksheet { nums, ops }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let mut worksheet = read_ranges_ids(file);
    worksheet.nums = transpose(worksheet.nums);

    info!("part1: {}", worksheet.get_total_part1());
    //
    // info!("part2: {}", worksheet.find_fresh_part2());

    //
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
