use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let digits = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<u8>, ParseIntError>>()?;

    Ok(digits)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let starting_fish = parse(lines?)?;

    // I spent a lot of time trying to figure out the scaling formula before
    // concluding there's a reason I didn't become a mathematician.
    // Let's just do it the fast way instead.

    // 9 days is the maximum cycle time.
    // All we need to keep track of is when each fish creates new fish.
    let mut fish = [0 as u64; 9];

    for x in starting_fish {
        fish[x as usize] += 1;
    }

    for day in 0..256 {
        if day == 80 {
            println!("Part 1: {}", fish.iter().sum::<u64>());
        }
        // In 7 days, the fish that created offspring today will do so again.
        // Since `fish` is both tracking the total amount of fish and the fish
        // spawns per cycle, we can kill two birds with one stone here.
        // You can think of fish[day % 9] as now containing today's new fish,
        // and of today's parents as moving to fish[(day + 7) % 9].
        fish[(day + 7) % 9] += fish[day % 9];
   }

    println!("Part 2: {}", fish.iter().sum::<u64>());
    Ok(())
}
