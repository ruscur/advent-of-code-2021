use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    let map = BTreeMap::from([('[', ']'), ('{', '}'), ('(', ')'), ('<', '>')]);
    let corrupted_points = BTreeMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let incomplete_points = BTreeMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut total = 0;
    let mut incomplete_scores = vec![];

    for line in lines {
        let mut opens = vec![];
        let mut corrupted = false;
        for c in line.chars() {
            if map.contains_key(&c) {
                opens.push(c);
            } else {
                let last = opens.pop().unwrap();
                if map[&last] != c {
                    total += corrupted_points[&c];
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            let mut completion_string = String::new();
            let mut completion_score: u64 = 0;
            while let Some(opener) = opens.pop() {
                let closer = map[&opener];
                completion_string.push(closer);
                completion_score *= 5;
                completion_score += incomplete_points[&closer];
            }
            incomplete_scores.push(completion_score);
        }
    }

    incomplete_scores.sort();

    println!("Part 1: {}", total);
    println!("Part 2: {}", incomplete_scores.get(incomplete_scores.len()/2).unwrap());

    Ok(())
}
