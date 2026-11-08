use std::{
    collections::HashSet,
    env,
    fmt::{self},
    fs::read_to_string,
    range,
    str::FromStr,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

struct Range {
    pub first: usize,
    pub last: usize,
}

impl Range {
    fn get_invalids_part1(&mut self) -> Vec<usize> {
        let mut invalids: Vec<usize> = vec![];
        for i in self.first..=self.last {
            let num_str = i.to_string();
            let len = num_str.len();
            if len % 2 != 0 {
                continue;
            }

            let (first, last) = num_str.split_at(len / 2);
            if first.eq(last) {
                invalids.push(i);
            }
        }
        invalids
    }

    fn get_invalids_part2(&mut self) -> HashSet<usize> {
        let mut invalids = HashSet::new();
        for i in self.first..=self.last {
            let num_str = i.to_string();
            let len = num_str.len();
            let mid = len / 2;
            let mut candidates = vec![];
            for i in 1..=mid {
                if len % i == 0 {
                    candidates.push(i);
                }
            }
            candidates.iter().for_each(|c| {
                let (first, last) = num_str.split_at(*c);
                if last.split(first).into_iter().all(|s| s.eq("")) {
                    invalids.insert(i);
                    return;
                }
            });
        }
        invalids
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
        r.get_invalids_part1().iter().for_each(|v| sum += v);
    });
    info!("part1: {sum}");

    let mut sum = 0;
    ranges.iter_mut().for_each(|r| {
        r.get_invalids_part2().iter().for_each(|v| sum += v);
    });

    info!("part2: {sum}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
