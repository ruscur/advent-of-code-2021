use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let digits = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u16>())
        .collect::<Result<Vec<u16>, ParseIntError>>()?;

    Ok(digits)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let mut crabs = parse(lines?)?;

    crabs.sort();

    let part1 = crabs.iter().fold(u64::MAX, |best, x| {
        let distance: u64 = crabs
            .iter()
            .map(|y| (*x as i32 - *y as i32).abs() as u64)
            .sum();
        if distance < best {
            distance
        } else {
            best
        }
    });

    println!("Part 1: {}", part1);

    let part2 = (0..*crabs.iter().last().unwrap()).fold(u64::MAX, |best, x| {
        let fuel: u64 = crabs
            .iter()
            .map(|y| {
                let distance = (x as i32 - *y as i32).abs() as u64;
                // yes I googled the formula for triangular numbers.
                (distance * (distance + 1)) / 2
            })
            .sum();
        if fuel < best {
            fuel
        } else {
            best
        }
    });

    println!("Part 2: {}", part2);

    Ok(())
}
