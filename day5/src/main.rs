use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
enum Orientation {
    Vertical,
    Horizontal,
    Diagonal,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    orientation: Orientation,
}

fn parse(lines: Vec<String>) -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let mut pairs: Vec<Line> = vec![];

    // Can you tell my parsing rigidity is getting lazier as we go? lol
    for line in lines {
        let line_parts: Vec<u16> = line
            .split(" -> ")
            .map(|x| x.split(","))
            .flatten()
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let mut iter = line_parts.into_iter();

        let start = Point {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        };
        let end = Point {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
        };

        // line.start should be lower than line.end; for diag, we make the lower x value the start
        let (lower, higher, orientation) = if start.y == end.y {
            let (lower, higher) = if start.x < end.x {
                (start, end)
            } else {
                (end, start)
            };
            (lower, higher, Orientation::Horizontal)
        } else if start.x == end.x {
            let (lower, higher) = if start.y < end.y {
                (start, end)
            } else {
                (end, start)
            };
            (lower, higher, Orientation::Vertical)
        } else {
            let (lower, higher) = if start.x < end.x {
                (start, end)
            } else {
                (end, start)
            };
            (lower, higher, Orientation::Diagonal)
        };

        let line = Line {
            start: lower,
            end: higher,
            orientation,
        };

        pairs.push(line);
    }
    Ok(pairs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let data = parse(lines?)?;

    // LAZY TO THE MAX
    // we can definitely be more space efficient than this, but it's Sunday, cmon
    let mut grid = [[0u16; 999]; 999];
    assert!(data.len() > 0);

    for line in data {
        match line.orientation {
            Orientation::Vertical => {
                for y in line.start.y..line.end.y + 1 {
                    grid[(y as usize)][(line.start.x as usize)] += 1;
                }
            }
            Orientation::Horizontal => {
                for x in line.start.x..line.end.x + 1 {
                    grid[(line.start.y as usize)][(x as usize)] += 1;
                }
            }
            Orientation::Diagonal => {
                // We know our x value will be lower from parsing, so we just need
                // to figure out the y direction.
                let y_inc = if line.start.y < line.end.y { 1 } else { -1 };
                let mut y = line.start.y as i16;
                for x in line.start.x..line.end.x + 1 {
                    // Just doing this so we can split parts 1 and 2.
                    grid[(y as usize)][(x as usize)] += 0xff;
                    y += y_inc;
                }
            }
        }
    }

    let part_1 = grid.iter().flatten().filter(|x| (**x % 0xff) >= 2).count();
    println!("Part 1: {}", part_1);
    let part_2 = grid
        .iter()
        .flatten()
        .map(|x| {
            if *x < 0xff {
                *x
            } else {
                // un-"hash" our diagonal matches, and add them to the count
                let diag_matches = *x / 0xff;
                (*x % 0xff) + diag_matches
            }
        })
        .filter(|x| *x >= 2)
        .count();
    println!("Part 2: {}", part_2);

    Ok(())
}
