use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<BTreeMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();

    lines.iter().for_each(|line| {
        if let Some((a, b)) = line.split_once("-") {
            graph
                .entry(String::from(a))
                .or_insert(vec![])
                .push(String::from(b));
            graph
                .entry(String::from(b))
                .or_insert(vec![])
                .push(String::from(a));
        }
    });

    Ok(graph)
}

fn is_lower(name: &String) -> bool {
    name.chars().filter(|c| c.is_lowercase()).count() == name.len()
}

fn explore(
    graph: &BTreeMap<String, Vec<String>>,
    path: Vec<String>,
    doubled: bool,
) -> Vec<Vec<String>> {
    let current_node = path.last().unwrap();
    if current_node == "end" {
        return vec![path];
    }

    let mut new_paths = vec![];

    for node in &graph[current_node] {
        let mut doubled = doubled;
        if is_lower(node) && path.contains(node) {
            if doubled || node == "start" {
                // We can't go there again, so skip.
                continue;
            } else {
                // This is our one double lowercase visit
                doubled = true;
            }
        }
        let mut new_path = path.clone();
        new_path.push(node.to_string());

        // Recursion two days in a row!
        new_paths.extend(explore(&graph, new_path, doubled));
    }

    new_paths
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    let graph = parse(lines)?;

    let paths = explore(&graph, vec![String::from("start")], true);

    println!("Part 1: {}", paths.len());

    let paths = explore(&graph, vec![String::from("start")], false);

    println!("Part 2: {}", paths.len());

    Ok(())
}
