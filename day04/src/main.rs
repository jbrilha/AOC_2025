use std::{
    env,
    fmt::{self},
    fs::read_to_string,
    str::FromStr,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

struct Grid {
    pub paper: Vec<Vec<bool>>,
    pub count: usize,
}

impl Grid {
    fn find_paper_rolls_part1(&mut self) {
        for i in 0..self.paper.len() {
            for j in 0..self.paper[i].len() {
                if self.paper[i][j] && self.can_reach(i, j) {
                    self.count += 1;
                }
            }
        }
    }

    fn can_reach(&self, row: usize, col: usize) -> bool {
        let mut n = 0;

        for i in -1..=1 {
            if let Some(r) = row.checked_add_signed(i) {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if let Some(c) = col.checked_add_signed(j) {
                        if let Some(rr) = self.paper.get(r) {
                            if let Some(cc) = rr.get(c) {
                                if *cc {
                                    n += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        n < 4
    }

    fn find_paper_rolls_part2(&mut self) {
        let mut removed = false;
        for i in 0..self.paper.len() {
            for j in 0..self.paper[i].len() {
                if self.paper[i][j] && self.can_reach(i, j) {
                    self.count += 1;
                    self.paper[i][j] = false;
                    removed = true;
                }
            }
        }

        if removed {
            self.find_paper_rolls_part2();
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.paper.iter().for_each(|l| {
            l.iter().for_each(|p| {
                write!(f, "{p} ").unwrap();
            });
            write!(f, "\n").unwrap();
        });

        write!(f, "reachable: {}", self.count)
    }
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| if c.eq(&'@') { true } else { false })
        .collect()
}

fn read_grid(filename: &str) -> Grid {
    let paper = read_to_string(filename)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();

    Grid { paper, count: 0 }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let mut grid = read_grid(file);

    grid.find_paper_rolls_part1();
    let sum = grid.count;
    info!("part1: {sum}");

    grid.count = 0;

    grid.find_paper_rolls_part2();
    let sum = grid.count;
    info!("part2: {sum}");

    //
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
