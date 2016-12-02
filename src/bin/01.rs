extern crate latent_catholicism;

use std::str::FromStr;
use std::collections::HashSet;
use latent_catholicism::one::ParseError;

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Debug)]
enum InstructionDirection {
    Left,
    Right,
}

impl FromStr for InstructionDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(InstructionDirection::Left),
            "R" => Ok(InstructionDirection::Right),
            _ => Err(String::from("must be L or R")),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Instruction {
    direction: InstructionDirection,
    distance: u32,
}

struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();

        for bit in s.split(", ") {
            let direction = bit[0..1].parse()?;
            let distance: u32 = bit[1..].parse()?;

            let instruction = Instruction {
                direction: direction,
                distance: distance,
            };
            
            v.push(instruction);
        }

        Ok(Instructions(v))
    }
}

struct PositionCache {
    cache: HashSet<(i64, i64)>,
    duplicate: Option<(i64, i64)>,
}

impl PositionCache {
    fn new() -> PositionCache {
        PositionCache {
            cache: HashSet::new(),
            duplicate: None,
        }
    }

    fn update_x(&mut self, position: (i64, i64), distance: i64) {
        for count in 0..distance.abs() {
            let seen_position = if distance > 0 {
                (position.0 + count, position.1)
            } else {
                (position.0 - count, position.1)
            };

            println!("visiting: {:?}", seen_position);

            if self.duplicate.is_none() {
                if self.cache.contains(&seen_position) {
                    self.duplicate = Some(seen_position);
                } else {
                    self.cache.insert(seen_position);
                }
            }
        }
    }

    fn update_y(&mut self, position: (i64, i64), distance: i64) {
        for count in 0..distance.abs() {
            let seen_position = if distance > 0 {
                (position.0, position.1 + count)
            } else {
                (position.0, position.1 - count)
            };

            println!("visiting: {:?}", seen_position);

            if self.duplicate.is_none() {
                if self.cache.contains(&seen_position) {
                    self.duplicate = Some(seen_position);
                } else {
                    self.cache.insert(seen_position);
                }
            }
        }
    }
}

fn main() {
    let input = "L1, L3, L5, L3, R1, L4, L5, R1, R3, L5, R1, L3, L2, L3, R2, R2, L3, L3, R1, L2, R1, L3, L2, R4, R2, L5, R4, L5, R4, L2, R3, L2, R4, R1, L5, L4, R1, L2, R3, R1, R2, L4, R1, L2, R3, L2, L3, R5, L192, R4, L5, R4, L1, R4, L4, R2, L5, R45, L2, L5, R4, R5, L3, R5, R77, R2, R5, L5, R1, R4, L4, L4, R2, L4, L1, R191, R1, L1, L2, L2, L4, L3, R1, L3, R1, R5, R3, L1, L4, L2, L3, L1, L1, R5, L4, R1, L3, R1, L2, R1, R4, R5, L4, L2, R4, R5, L1, L2, R3, L4, R2, R2, R3, L2, L3, L5, R3, R1, L4, L3, R4, R2, R2, R2, R1, L4, R4, R1, R2, R1, L2, L2, R4, L1, L2, R3, L3, L5, L4, R4, L3, L1, L5, L3, L5, R5, L5, L4, L2, R1, L2, L4, L2, L4, L1, R4, R4, R5, R1, L4, R2, L4, L2, L4, R2, L4, L1, L2, R1, R4, R3, R2, R2, R5, L1, L2";

    let directions: Instructions = input.parse().expect("parse failed");

    // Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
    // R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
    // R5, L5, R5, R3 leaves you 12 blocks away.

    let mut position = (0i64, 0i64);
    let mut direction = Direction::Up;
    let mut cache = PositionCache::new();

    for instruction in directions.0 {
        direction = match (instruction.direction, direction) {
            (InstructionDirection::Left, Direction::Left) => {
                let distance = instruction.distance as i64;

                cache.update_y(position, -distance);

                position.1 -= distance;

                Direction::Down
            },
            (InstructionDirection::Left, Direction::Right) => {
                let distance = instruction.distance as i64;

                cache.update_y(position, distance);

                position.1 += distance;

                Direction::Up
            },
            (InstructionDirection::Left, Direction::Up) => {
                let distance = instruction.distance as i64;

                cache.update_x(position, -distance);

                position.0 -= distance;

                Direction::Left
            },
            (InstructionDirection::Left, Direction::Down) => {
                let distance = instruction.distance as i64;

                cache.update_x(position, distance);

                position.0 += distance;

                Direction::Right
            },
            (InstructionDirection::Right, Direction::Left) => {
                let distance = instruction.distance as i64;

                cache.update_y(position, distance);

                position.1 += distance;

                Direction::Up
            },
            (InstructionDirection::Right, Direction::Right) => {
                let distance = instruction.distance as i64;

                cache.update_y(position, -distance);

                position.1 -= distance;

                Direction::Down
            },
            (InstructionDirection::Right, Direction::Up) => {
                let distance = instruction.distance as i64;

                cache.update_x(position, distance);

                position.0 += distance;

                Direction::Right
            },
            (InstructionDirection::Right, Direction::Down) => {
                let distance = instruction.distance as i64;

                cache.update_x(position, -distance);

                position.0 -= distance;

                Direction::Left
            },
        }
    }

    println!("end spot: ({},{})", position.0, position.1);
    println!("blocks: {}", position.0.abs() + position.1.abs());

    if let Some(seen) = cache.duplicate {
        println!("seen twice: {:?}", seen);
        println!("blocks: {}", seen.0.abs() + seen.1.abs())
    }
}