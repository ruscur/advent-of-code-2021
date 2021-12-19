use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Eq for Position {}

impl Position {
    fn new(x: isize, y: isize, z: isize) -> Position {
        Position { x, y, z }
    }

    fn add(&self, other: &Self) -> Position {
        Position {
            x: other.x + self.x,
            y: other.y + self.y,
            z: other.z + self.z,
        }
    }

    fn sub(&self, other: &Self) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn abs(&self) -> Position {
        Position {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn sum(&self) -> isize {
        self.x + self.y + self.z
    }

    fn permute(&self, val: usize) -> Position {
        let mut permutations = vec![];
        // This is double the amount of permutations but I have no clue
        // which ones are unused.
        for x in [1, -1] {
            for y in [1, -1] {
                for z in [1, -1] {
                    let x = self.x * x;
                    let y = self.y * y;
                    let z = self.z * z;
                    permutations.push(Position::new(x, y, z));
                    permutations.push(Position::new(x, z, y));
                    permutations.push(Position::new(y, z, x));
                    permutations.push(Position::new(y, x, z));
                    permutations.push(Position::new(z, x, y));
                    permutations.push(Position::new(z, y, x));
                }
            }
        }
        permutations[val].clone()
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Vec<Position>>, Box<dyn std::error::Error>> {
    let mut scanners = vec![];
    let mut positions = vec![];

    for line in lines {
        if line.starts_with("---") {
            continue;
        } else if line.is_empty() {
            scanners.push(positions);
            positions = vec![];
        } else {
            let values = line
                .split(",")
                .map(|n| n.parse::<isize>())
                .collect::<Result<Vec<isize>, ParseIntError>>()?;
            positions.push(Position {
                x: values[0],
                y: values[1],
                z: values[2],
            });
        }
    }

    if !positions.is_empty() {
        scanners.push(positions);
    }

    Ok(scanners)
}

// Never used HashSet before, it's a bit faster and saves checking for duplicates
fn merge(probes: &mut HashSet<Position>, other: &Vec<Position>) -> Option<Position> {
    for i in 0..48 {
        let adjusted = other
            .iter()
            .map(|v| v.permute(i))
            .collect::<HashSet<Position>>();
        let distances: Vec<Position> = probes
            .iter()
            .map(|x| adjusted.iter().map(|y| (x.clone(), y.clone())))
            .flatten()
            .map(|(x, y)| x.sub(&y))
            .collect();

        for offset in distances {
            let adjusted = adjusted
                .iter()
                .map(|x| x.add(&offset))
                .collect::<Vec<Position>>();

            if adjusted.iter().filter(|x| probes.contains(x)).count() >= 12 {
                probes.extend(adjusted);
                return Some(offset);
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let mut scanners: Vec<Vec<Position>> = parse(lines?)?;

    let mut probes = HashSet::new();

    probes.extend(scanners[0].clone());
    scanners.remove(0);

    let mut distances = vec![];

    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some(distance) = merge(&mut probes, &scanners[i]) {
                distances.push(distance);
                scanners.remove(i);
            }
        }
    }

    println!("Part 1: {}", probes.len());

    println!(
        "Part 2: {}",
        distances
            .iter()
            .filter_map(|x| distances.iter().map(|y| x.sub(&y).abs().sum()).max())
            .max()
            .unwrap()
    );

    Ok(())
}
