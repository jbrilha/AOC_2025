use std::{
    cmp::max,
    env,
    fmt::{self},
    fs::read_to_string,
    str::FromStr,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

struct Database {
    pub ranges: Vec<(usize, usize)>,
    pub ids: Vec<usize>,
}

impl Database {
    fn find_fresh_part1(&mut self) -> usize {
        let mut fresh = 0;
        self.ids.iter().for_each(|id| {
            if self
                .ranges
                .iter()
                .find_map(|(low, high)| {
                    if (low..=high).contains(&id) {
                        return Some(());
                    }
                    None
                })
                .is_some()
            {
                fresh += 1;
            }
        });

        fresh
    }

    fn find_fresh_part2(&mut self) -> usize {
        let mut fresh = 0;

        self.ranges.sort_by(|l, r| l.cmp(r));

        let mut sets: Vec<(usize, usize)> = vec![];

        self.ranges.iter().for_each(|&(l, r)| {
            if sets.is_empty() {
                sets.push((l, r));
                return;
            }

            let last = sets.last_mut().unwrap();
            if (r > last.1 && l <= last.1) || (l >= last.0 && r <= last.1) {
                last.1 = max(r, last.1);
            } else {
                sets.push((l, r));
            }
        });

        sets.iter().for_each(|(l, r)| {
            fresh += r - l + 1;
        });

        fresh
    }
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ranges.iter().for_each(|r| {
            write!(f, "{}-{}\n", r.0, r.1).unwrap();
        });

        self.ids.iter().for_each(|i| {
            write!(f, "{i}\n").unwrap();
        });

        Ok(())
    }
}

fn parse_range(line: &str) -> (usize, usize) {
    let parts = line.split('-').collect::<Vec<&str>>();
    let l = usize::from_str(parts.first().unwrap()).unwrap();
    let r = usize::from_str(parts.last().unwrap()).unwrap();
    (l, r)
}

fn read_ranges_ids(filename: &str) -> Database {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut swap = false;

    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            if l.is_empty() {
                swap = true;
                return;
            }

            if !swap {
                ranges.push(parse_range(l))
            } else {
                ids.push(usize::from_str(l).unwrap());
            }
        })
        .count();

    Database { ranges, ids }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let mut db = read_ranges_ids(file);

    info!("part1: {}", db.find_fresh_part1());

    info!("part2: {}", db.find_fresh_part2());

    //
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
