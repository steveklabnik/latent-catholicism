mod error;

use std::str::FromStr;
use error::ParseError;

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

fn main() {
    let input = "L1, L3, L5, L3, R1, L4, L5, R1, R3, L5, R1, L3, L2, L3, R2, R2, L3, L3, R1, L2, R1, L3, L2, R4, R2, L5, R4, L5, R4, L2, R3, L2, R4, R1, L5, L4, R1, L2, R3, R1, R2, L4, R1, L2, R3, L2, L3, R5, L192, R4, L5, R4, L1, R4, L4, R2, L5, R45, L2, L5, R4, R5, L3, R5, R77, R2, R5, L5, R1, R4, L4, L4, R2, L4, L1, R191, R1, L1, L2, L2, L4, L3, R1, L3, R1, R5, R3, L1, L4, L2, L3, L1, L1, R5, L4, R1, L3, R1, L2, R1, R4, R5, L4, L2, R4, R5, L1, L2, R3, L4, R2, R2, R3, L2, L3, L5, R3, R1, L4, L3, R4, R2, R2, R2, R1, L4, R4, R1, R2, R1, L2, L2, R4, L1, L2, R3, L3, L5, L4, R4, L3, L1, L5, L3, L5, R5, L5, L4, L2, R1, L2, L4, L2, L4, L1, R4, R4, R5, R1, L4, R2, L4, L2, L4, R2, L4, L1, L2, R1, R4, R3, R2, R2, R5, L1, L2";

    let directions: Instructions = input.parse().expect("parse failed");

    assert_eq!(directions.0.last(), Some(&Instruction { direction: InstructionDirection::Left, distance: 2}));

    // Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
    // R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
    // R5, L5, R5, R3 leaves you 12 blocks away.

    let mut position = (0i64, 0i64);
    let mut direction = Direction::Up;

    for instruction in directions.0 {
        direction = match (instruction.direction, direction) {
            (InstructionDirection::Left, Direction::Left) => {
                position.1 -= instruction.distance as i64;

                Direction::Down
            },
            (InstructionDirection::Left, Direction::Right) => {
                position.0 += instruction.distance as i64;

                Direction::Up
            },
            (InstructionDirection::Left, Direction::Up) => {
                position.0 -= instruction.distance as i64;

                Direction::Left
            },
            (InstructionDirection::Left, Direction::Down) => {
                position.0 += instruction.distance as i64;

                Direction::Right
            },
            (InstructionDirection::Right, Direction::Left) => {
                position.1 += instruction.distance as i64;

                Direction::Up
            },
            (InstructionDirection::Right, Direction::Right) => {
                position.1 -= instruction.distance as i64;

                Direction::Down
            },
            (InstructionDirection::Right, Direction::Up) => {
                position.0 += instruction.distance as i64;

                Direction::Right
            },
            (InstructionDirection::Right, Direction::Down) => {
                position.0 -= instruction.distance as i64;

                Direction::Left
            },
        }
    }

    println!("({},{})", position.0, position.1);
    println!("blocks: {}", position.0.abs() + position.1.abs())
}