use std::{
    env,
    fmt::{self},
    fs::read_to_string,
    str::FromStr,
};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

struct Bank {
    pub batteries: Vec<usize>,
    pub joltage: usize,
}

impl Bank {
    fn calc_joltage_part1(&mut self) {
        let len = self.batteries.len();

        let maxl = self
            .batteries
            .iter()
            .rev()
            .enumerate()
            .max_by_key(|&(idx, val)| if idx > 0 { *val } else { 0 })
            .unwrap();

        let maxr = self.batteries.iter().skip(len - maxl.0).max().unwrap();

        self.joltage = maxl.1 * 10 + maxr;
    }

    fn calc_joltage_part2(&mut self) {
        self.joltage = 0;
        let len = self.batteries.len();

        let mut vals: Vec<usize> = vec![];

        let mut need = 12;

        let mut skip = 0;
        while need > 0 {
            let (idx, max) = self
                .batteries
                .iter()
                .skip(skip)
                .rev()
                .skip(need - 1)
                .enumerate()
                .max_by_key(|&(_, val)| *val)
                .unwrap();

            vals.push(*max);

            skip = len - (need + idx) + 1;
            need -= 1;
        }

        let mut powr = 12;
        vals.iter().for_each(|v| {
            powr -= 1;
            self.joltage += (*v) as usize * 10_usize.pow(powr);
        });
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(input: &str) -> Result<Bank, Self::Err> {
        let mut batteries: Vec<usize> = vec![];
        input
            .chars()
            .for_each(|c| batteries.push(c.to_digit(10).unwrap() as usize));

        Ok(Bank {
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
        write!(f, ", joltage: {}", self.joltage)
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
        r.calc_joltage_part1();
        sum += r.joltage;
    });
    info!("part1: {sum}");

    let mut sum = 0;
    banks.iter_mut().for_each(|r| {
        r.calc_joltage_part2();
        sum += r.joltage;
    });
    info!("part2: {sum}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_ok() {}
}
