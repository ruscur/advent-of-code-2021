use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let digits = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect::<Vec<u8>>();

    Ok(digits)
}

// This is hard-coded for a 10x10 grid
fn get_adjacents(pos: usize) -> Vec<usize> {
    let mut adjacents = vec![];

    if pos >= 11 && (pos % 10 > 0) {
        // top left
        adjacents.push(pos - 11);
    }

    if pos >= 10 {
        // above
        adjacents.push(pos - 10);
    }

    if pos >= 10 && (pos % 10 < 9) {
        // top right
        adjacents.push(pos - 9);
    }

    if pos % 10 > 0 {
        // left
        adjacents.push(pos - 1);
    }

    if pos % 10 < 9 {
        // right
        adjacents.push(pos + 1);
    }

    if pos <= 89 && (pos % 10 > 0) {
        // bottom left
        adjacents.push(pos + 9);
    }

    if pos <= 89 {
        // below
        adjacents.push(pos + 10);
    }

    if pos <= 88 && (pos % 10 < 9) {
        // bottom right
        adjacents.push(pos + 11);
    }

    adjacents
}

fn flash(octopi: &mut Vec<u8>, pos: usize) {
    let adjacents = get_adjacents(pos);

    for adj_pos in adjacents {
        octopi[adj_pos] += 1;
        // It's important that we only check 10, because each octopus can only flash once.
        if octopi[adj_pos] == 10 {
            // Look ma, I did a recursion!
            flash(octopi, adj_pos);
        }
    }
}

fn process_step(octopi: Vec<u8>) -> Vec<u8> {
    let mut octopi: Vec<u8> = octopi.iter().map(|x| x + 1).collect();

    let flashers: Vec<usize> = octopi
        .iter()
        .enumerate()
        .filter(|&(_, x)| *x == 10)
        .map(|(pos, _)| pos)
        .collect();

    for pos in flashers {
        flash(&mut octopi, pos);
    }

    octopi
        .iter()
        .map(|&x| if x >= 10 { 0 } else { x })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let mut octopi = parse(lines?)?;
    let mut flash_count = 0;

    for _ in 0..100 {
        octopi = process_step(octopi);
        flash_count += octopi.iter().filter(|&x| *x == 0).count();
    }

    println!("Part 1: {}", flash_count);

    let mut steps = 100;
    while octopi.iter().filter(|&x| *x == 0).count() != octopi.len() {
        octopi = process_step(octopi);
        steps += 1;
    }

    println!("Part 2: {}", steps);

    Ok(())
}
