use std::{
    collections::HashSet,
    env,
    fmt::{self},
    fs::read_to_string,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

struct Manifold {
    pub start: (usize, usize),
    pub len: usize,
    pub splits: Vec<Vec<(usize, usize)>>,
}

impl Manifold {
    fn count_splits_part1(&mut self) -> usize {
        let mut beams = HashSet::new();
        beams.insert(self.start.0);
        let mut count = 0;

        self.splits.iter().for_each(|l| {
            l.iter().for_each(|s| {
                if beams.remove(&s.0) {
                    count += 1;
                    beams.insert(s.0 - 1);
                    beams.insert(s.0 + 1);
                }
            });
        });

        count
    }

    fn count_splits_part2(&mut self) -> usize {
        0
    }
}

impl fmt::Display for Manifold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S @ {}:{} | {}\n", self.start.0, self.start.1, self.len).unwrap();
        self.splits.iter().for_each(|s| {
            s.iter().for_each(|p| {
                write!(f, "split @ {}:{}", p.0, p.1).unwrap();
            });

            write!(f, "\n").unwrap();
        });

        Ok(())
    }
}

fn read_manifold(filename: &str) -> Manifold {
    let mut splits = vec![];

    let input = read_to_string(filename).unwrap();
    let mut lines = input.lines();

    let start = (lines.next().unwrap().find('S').unwrap(), 0);
    let len = start.0 * 2;

    lines.enumerate().for_each(|(idx, l)| {
        let mut inner = vec![];

        l.char_indices().for_each(|(cidx, c)| {
            if c.eq(&'^') {
                inner.push((cidx, idx + 1));
            }
        });

        if !inner.is_empty() {
            splits.push(inner);
        }
    });

    Manifold { start, len, splits }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let mut manifold = read_manifold(file);
    // info!("{manifold}");

    info!("part1: {}", manifold.count_splits_part1());

    info!("part2: {}", manifold.count_splits_part2());

    //
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
