use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

fn parse(buf: &mut BufReader<File>) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut depths: Vec<u64> = vec!();

    // We could check for increases here so we don't have to read the vec again.
    // That is more performant, but makes it harder to make changes later.
    for line in buf.lines() {
        depths.push(line?.parse::<u64>()?);
    }
    Ok(depths)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let depths = parse(&mut reader)?;

    assert!(depths.len() > 0);

    // (199, 200), (200, 208), (208, 210) ...
    let depth_pairs = depths.iter().zip(depths.iter().skip(1));

    println!("Part 1: {}", depth_pairs.clone().filter(|(x, y)| x < y).count());

    // ((199, 200), 208), ((200, 208), 210), ((208, 210), 200) ...
    let depth_triplets = depth_pairs.zip(depths.iter().skip(2));
    // 607, 618, 618, 617 ...
    let windows: Vec<u64> = depth_triplets.map(|((x, y), z)| x + y + z).collect();
    // (607, 618), (618, 618), (618, 617) ...
    let window_pairs = windows.iter().zip(windows.iter().skip(1));

    println!("Part 2: {}", window_pairs.filter(|(x, y)| x < y).count());

    Ok(())
}
