use std::{
    env,
    fmt::{self},
    fs::read_to_string,
    str::FromStr,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

const MAX: isize = 99;
const MIN: isize = 0;
const BRUTE_FORCE: bool = false;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

struct Rotation {
    pub direction: Direction,
    pub distance: isize,
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(input: &str) -> Result<Rotation, Self::Err> {
        let (dir, dist) = input.split_at(1);
        let direction = Direction::from_str(dir).unwrap();
        let distance: isize = dist.parse().unwrap();

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

struct Dial {
    pub position: isize,
    pub at_zero: isize,
    pub passed_zero: isize,
    pub last_count: isize,
    pub wrapped: bool,
}

impl Dial {
    fn new(position: isize) -> Self {
        Self {
            position,
            at_zero: 0,
            passed_zero: 0,
            last_count: 0,
            wrapped: false,
        }
    }

    fn apply_rotation(&mut self, rotation: &Rotation) {
        let was_zero = self.position == 0;
        let rem = rotation.distance.rem_euclid(MAX + 1);
        let mut turns = rotation.distance.div_euclid(MAX + 1);

        self.wrapped = match rotation.direction {
            Direction::Left => self.turn_left(rem),
            Direction::Right => self.turn_right(rem),
        } && !was_zero;

        if self.position == 0 {
            self.at_zero += 1;
            if turns >= 1 {
                self.passed_zero += turns;
            }
        } else if !self.wrapped {
            self.passed_zero += turns;
        } else {
            turns += 1;
            self.passed_zero += turns;
        }

        self.last_count = turns;
    }

    fn apply_rotation_bruteforce(&mut self, rotation: &Rotation) {
        let mut wraps = 0;
        for _ in 0..rotation.distance {
            match rotation.direction {
                Direction::Left => self.turn_left(1),
                Direction::Right => self.turn_right(1),
            };

            if self.position == 0 {
                self.passed_zero += 1;
                wraps += 1;
            }
        }

        self.last_count = wraps;
        self.wrapped = wraps > 0;

        if self.position == 0 {
            self.at_zero += 1;
        }
    }

    fn turn_left(&mut self, rem: isize) -> bool {
        if self.position.saturating_sub(rem) < MIN {
            let delta = rem.saturating_sub(self.position);
            self.position = MAX - delta + 1;
            true
        } else {
            self.position = self.position.saturating_sub(rem);
            false
        }
    }

    fn turn_right(&mut self, rem: isize) -> bool {
        if self.position.saturating_add(rem) > MAX {
            let delta = rem.saturating_sub(MAX - self.position);
            self.position = MIN + delta - 1;
            true
        } else {
            self.position = self.position.saturating_add(rem);
            false
        }
    }
}

fn read_rotations(filename: &str) -> Vec<Rotation> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| Rotation::from_str(s).unwrap())
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

    let rotations = read_rotations(file);

    let mut dial = Dial::new(50);

    rotations.iter().for_each(|r| {
        if BRUTE_FORCE {
            dial.apply_rotation_bruteforce(r);
        } else {
            dial.apply_rotation(r);
        }

        if dial.position != 0 && dial.wrapped {
            info!(
                "The dial is rotated {} to point at {}; \
                during this rotation, it points at 0 {} times",
                r, dial.position, dial.last_count
            )
        } else {
            info!("The dial is rotated {} to point at {}", r, dial.position)
        }

        dial.wrapped = false;
    });

    if !BRUTE_FORCE {
        dial.passed_zero += dial.at_zero
    }
    info!("at_zero: \t{} (part1)", dial.at_zero);
    info!("passed_zero: \t{} (part2)", dial.passed_zero);
}

#[cfg(test)]
mod tests {
    use crate::{
        Dial,
        Direction::{Left, Right},
        MAX, MIN, Rotation,
    };

    #[test]
    fn example_ok() {
        let mut dial = Dial::new(11);
        dial.apply_rotation(&Rotation {
            direction: Right,
            distance: 8,
        });
        assert_eq!(dial.position, 19);

        dial.apply_rotation(&Rotation {
            direction: Left,
            distance: 19,
        });
        assert_eq!(dial.position, 0);

        let mut dial = Dial::new(MIN);
        dial.apply_rotation(&Rotation {
            direction: Left,
            distance: 1,
        });
        assert_eq!(dial.position, MAX);

        let mut dial = Dial::new(MAX);
        dial.apply_rotation(&Rotation {
            direction: Right,
            distance: 1,
        });
        assert_eq!(dial.position, MIN);

        let mut dial = Dial::new(5);
        dial.apply_rotation(&Rotation {
            direction: Left,
            distance: 10,
        });
        assert_eq!(dial.position, 95);

        dial.apply_rotation(&Rotation {
            direction: Right,
            distance: 5,
        });
        assert_eq!(dial.position, 0);
    }
}
