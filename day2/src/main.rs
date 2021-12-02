use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
enum CommandParseError {
    Direction,
    Value,
    Command,
}

impl std::error::Error for CommandParseError {}

impl std::fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandParseError::Direction => write!(f, "failed to parse direction"),
            CommandParseError::Value => write!(f, "failed to parse value"),
            CommandParseError::Command => write!(f, "command line mangled, couldn't parse"),
        }
    }
}

impl FromStr for Direction {
    type Err = CommandParseError;

    fn from_str(buf: &str) -> Result<Self, Self::Err> {
        match buf {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(CommandParseError::Direction),
        }
    }
}

#[derive(Debug)]
struct Command {
    value: u8,
    direction: Direction,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(buf: &str) -> Result<Self, Self::Err> {
        let mut parts = buf.split(" ");

        let dir = parts
            .next()
            .ok_or_else(|| Self::Err::Command)?
            .parse::<Direction>()?;
        let val = parts
            .next()
            .ok_or_else(|| Self::Err::Command)?
            .parse::<u8>()
            .map_err(|_| Self::Err::Value)?;

        match parts.next() {
            Some(_) => Err(Self::Err::Command),
            None => Ok(Command {
                value: val,
                direction: dir,
            }),
        }
    }
}

struct Submarine {
    horiz: i64,
    depth: i64,
    aim: i64,
}

impl Submarine {
    fn move_up(&self, val: u8) -> Submarine {
        Submarine {
            horiz: self.horiz,
            depth: self.depth,
            aim: self.aim - (val as i64),
        }
    }
    fn move_down(&self, val: u8) -> Submarine {
        Submarine {
            horiz: self.horiz,
            depth: self.depth,
            aim: self.aim + (val as i64),
        }
    }
    fn move_forward(&self, val: u8) -> Submarine {
        Submarine {
            horiz: self.horiz + (val as i64),
            depth: self.depth + (self.aim * (val as i64)),
            aim: self.aim,
        }
    }
}

fn parse(buf: &mut BufReader<File>) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    let mut commands: Vec<Command> = vec![];

    // as before, we're treating parsing separately from solution logic
    for line in buf.lines() {
        commands.push(line?.parse::<Command>()?);
    }
    Ok(commands)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let commands = parse(&mut reader)?;

    assert!(commands.len() > 0);

    let final_position = commands.iter().fold(
        Submarine {
            depth: 0,
            horiz: 0,
            aim: 0,
        },
        |pos, com| match com.direction {
            Direction::Up => pos.move_up(com.value),
            Direction::Down => pos.move_down(com.value),
            Direction::Forward => pos.move_forward(com.value),
        },
    );

    // Part 1's depth is the same as part 2's aim
    println!(
        "Part 1: horiz {} * depth {} = {}",
        final_position.horiz,
        final_position.aim,
        final_position.horiz * final_position.aim
    );

    println!(
        "Part 2: horiz {} * depth {} = {}",
        final_position.horiz,
        final_position.depth,
        final_position.horiz * final_position.depth
    );

    Ok(())
}
