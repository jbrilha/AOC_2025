use std::{
    env,
    fmt::{self},
    fs::read_to_string,
    str::FromStr,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

struct Bank {
    pub idxs: (usize, usize),
    pub batteries: Vec<u32>,
    pub joltage: u32,
}

impl Bank {
    fn calc_joltage(&mut self) {
        let len = self.batteries.len();

        let maxl = self
            .batteries
            .iter()
            .rev()
            .enumerate()
            .max_by_key(|&(idx, val)| if idx > 0 { *val } else { 0 })
            .unwrap();
        error!("idx: {}, val: {}", maxl.0, maxl.1);

        let maxr = self
            .batteries
            .iter()
            .skip(len - maxl.0)
            .enumerate()
            .max_by_key(|&(_idx, val)| val)
            .unwrap();

        error!("idx: {}, val: {}", maxr.0, maxr.1);

        self.joltage = maxl.1 * 10 + maxr.1;
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(input: &str) -> Result<Bank, Self::Err> {
        let mut batteries = vec![];
        input
            .chars()
            .for_each(|c| batteries.push(c.to_digit(10).unwrap()));

        Ok(Bank {
            idxs: (0, 0),
            batteries,
            joltage: 0,
        })
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.batteries.iter().for_each(|b| {
            write!(f, "{b}").unwrap();
        });
        write!(
            f,
            " | {} and {} | {}",
            self.idxs.0, self.idxs.1, self.joltage
        )
    }
}

fn read_banks(filename: &str) -> Vec<Bank> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| Bank::from_str(s).unwrap())
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

    let mut banks = read_banks(file);

    let mut sum = 0;
    banks.iter_mut().for_each(|r| {
        r.calc_joltage();
        sum += r.joltage;
        info!("{r}");
    });
    info!("part1: {sum}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
