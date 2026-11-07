use std::{env, fmt, fs::read_to_string, str::FromStr};

const MAX: isize = 99;
const MIN: isize = 0;

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
    pub at_zero: usize,
}

impl Dial {
    fn new(position: isize) -> Self {
        Self {
            position,
            at_zero: 0,
        }
    }

    fn apply_rotation(&mut self, rotation: &Rotation) {
        match rotation.direction {
            Direction::Left => self.turn_left(rotation.distance),
            Direction::Right => self.turn_right(rotation.distance),
        }
    }

    fn turn_left(&mut self, distance: isize) {
        let rem = distance % (MAX + 1);

        if self.position.saturating_sub(rem) < MIN {
            let actual = rem.saturating_sub(self.position);
            self.position = MAX - actual + 1
        } else {
            self.position = self.position.saturating_sub(rem)
        }

        if self.position == 0 {
            self.at_zero += 1
        }
    }

    fn turn_right(&mut self, distance: isize) {
        let rem = distance % (MAX + 1);

        if self.position.saturating_add(rem) > MAX {
            let actual = rem.saturating_sub(MAX - self.position);
            self.position = MIN + actual - 1
        } else {
            self.position = self.position.saturating_add(rem)
        }

        if self.position == 0 {
            self.at_zero += 1
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
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let rotations = read_rotations(file);

    let mut dial = Dial::new(50);

    rotations.iter().for_each(|r| {
        dial.apply_rotation(r);
        println!("The dial is rotated {} to point at {}", r, dial.position)
    });

    println!("{}", dial.at_zero);
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
