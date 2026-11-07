use core::num;
use std::{
    env,
    fmt::{self},
    fs::read_to_string,
    str::FromStr,
};

use tracing::info;
use tracing_subscriber::EnvFilter;

struct Range {
    pub first: usize,
    pub last: usize,
}

impl Range {
    fn get_valids(&mut self) -> Vec<usize> {
        let mut valids: Vec<usize> = vec![];
        for i in self.first..=self.last {
            let num_str = i.to_string();
            let len = num_str.len();
            if len % 2 != 0 {
                continue;
            }

            let (first, last) = num_str.split_at(len / 2);
            if first.eq(last) {
                valids.push(i);
            }
        }
        valids
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(input: &str) -> Result<Range, Self::Err> {
        let mut parts = input.split('-').into_iter();
        let first_str = parts.next().unwrap();
        let last_str = parts.next().unwrap();
        let first = usize::from_str(first_str).unwrap();
        let last = usize::from_str(last_str).unwrap();

        Ok(Range { first, last })
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "from {} to {}", self.first, self.last)
    }
}

fn read_ranges(filename: &str) -> Vec<Range> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .map(|s| Range::from_str(s).unwrap())
        .collect()
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let mut ranges = read_ranges(file);

    let mut sum = 0;
    ranges.iter_mut().for_each(|r| {
        r.get_valids().iter().for_each(|v| sum += v);
    });

    info!("{sum}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
