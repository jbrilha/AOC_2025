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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Mul,
}

struct Worksheet {
    pub nums: Vec<Vec<usize>>,
    pub num_matrix: Vec<Vec<Vec<Option<u32>>>>,
    pub ops: Vec<Operation>,
    pub digits: u32,
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

    fn get_total_part2(&mut self) -> usize {
        let mut total = 0;

        self.num_matrix.iter().enumerate().for_each(|(idx, row)| {
            let op = self.ops[idx];
            let mut col_total: usize = match op {
                Operation::Sum => 0,
                Operation::Mul => 1,
            };
            transpose(row.clone()).iter().rev().for_each(|inner| {
                let mut num: usize = 0;
                let mut digits = self.digits;
                inner
                    .iter()
                    .map(|p| {
                        if let Some(n) = p {
                            num += (n * 10_u32.pow(digits - 1)) as usize;
                        } else {
                            num /= 10;
                        }
                        digits -= 1;
                    })
                    .count();

                match op {
                    Operation::Sum => col_total += num,
                    Operation::Mul => col_total *= num,
                };
            });
            total += col_total;
        });

        total
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

fn parse_nums_part2(counts: &Vec<usize>, line: &str) -> Vec<Vec<Option<u32>>> {
    let mut ll: Vec<Vec<Option<u32>>> = vec![];

    let mut skip = 0;
    counts.iter().for_each(|&co| {
        let mut digits = co;
        let mut inner = vec![];

        line.chars()
            .skip(skip)
            .take_while(|c| {
                inner.push(c.to_digit(10));

                digits -= 1;
                digits > 0
            })
            .count();

        ll.push(inner);

        skip += co + 1;
    });

    ll
}

fn parse_ops(line: &str) -> (Vec<Operation>, Vec<usize>) {
    let mut ops = vec![];
    let mut counts = vec![];
    let mut count = 0;
    let mut op: Option<Operation> = None;
    line.chars()
        .map(|c| match c {
            '+' => {
                if op.is_some() {
                    ops.push(op.unwrap());
                    counts.push(count);
                }
                op = Some(Operation::Sum);
                count = 0
            }
            '*' => {
                if op.is_some() {
                    ops.push(op.unwrap());
                    counts.push(count);
                }
                op = Some(Operation::Mul);
                count = 0
            }
            ' ' => count += 1,
            _ => panic!(),
        })
        .count();

    count += 1;
    ops.push(op.unwrap());
    counts.push(count);

    (ops, counts)
}

// totally not cheating: https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn rotate(matrix: &mut Vec<Vec<usize>>) {
    let n = matrix.len();

    for i in 0..n {
        for j in i + 1..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn read_ranges_ids(filename: &str) -> Worksheet {
    let mut nums = vec![];
    let mut ops = vec![];
    let mut counts = vec![];
    let mut num_matrix = vec![];

    let input = read_to_string(filename).unwrap();
    let mut lines = input.lines();
    let mut row_counts = 0;

    lines.next_back().map(|l| {
        (ops, counts) = parse_ops(l);
    });

    lines
        .map(|l| {
            row_counts += 1;
            nums.push(parse_nums(l));
            num_matrix.push(parse_nums_part2(&counts, l));
        })
        .count();

    Worksheet {
        nums: transpose(nums),
        num_matrix: transpose(num_matrix),
        ops,
        digits: row_counts,
    }
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

    info!("part1: {}", worksheet.get_total_part1());

    info!("part2: {}", worksheet.get_total_part2());

    //
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
