use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let digits = lines
        .iter()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .flatten()
        .collect::<Vec<u32>>();

    Ok(digits)
}

// How many more times will I write this in the coming weeks?
fn get_neighbours(pos: usize, size: usize) -> Vec<usize> {
    let axis_size = (size as f64).sqrt() as usize;
    let mut neighbours = vec![];
    if (pos + 1) % axis_size > 0 {
        neighbours.push(pos + 1);
    }

    if pos < size - axis_size {
        neighbours.push(pos + axis_size);
    }

    if pos >= axis_size {
        neighbours.push(pos - axis_size);
    }

    if pos % axis_size >= 1 {
        neighbours.push(pos - 1);
    }

    neighbours
}

// This turned out to not actually be needed but it was nice to debug with
fn find_actual_path(dist: &Vec<u32>) -> Vec<(usize, u32)> {
    let mut pos = dist.len() - 1;
    let mut path = vec![(pos, dist[pos])];

    loop {
        let neighbours = get_neighbours(pos, dist.len());
        let mut min_pos = usize::MAX;
        let mut min_score = u32::MAX;

        for pos in neighbours {
            if dist[pos] < min_score {
                min_pos = pos;
                min_score = dist[pos];
            }
        }
        path.push((min_pos, min_score));
        pos = min_pos;
        if pos == 0 {
            break;
        }
    }
    path.reverse();
    path
}

fn find_best_path(map: &Vec<u32>) -> u32 {
    let size = map.len();
    let mut distance = vec![u32::MAX - 1; size];

    distance[0] = 0;
    let mut changed = true;
    let mut iterations = 0;

    // Once we go an iteration without a modification, we're perfect
    while changed {
        changed = false;
        iterations += 1;
        for index in 0..size {
            for pos in get_neighbours(index, size) {
                if distance[pos] > distance[index] + map[pos] {
                    distance[pos] = distance[index] + map[pos];
                    changed = true;
                }
            }
        }
    }

    let path = find_actual_path(&distance);
    println!("Took {} iterations to find path of length {}", iterations, path.len());

    distance[size - 1]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let map = parse(lines?)?;

    println!("Part 1: {}", find_best_path(&map));

    // Having a flattened array really ruined my brain with all the indexing.
    let mut map_but_more_big = vec![0; map.len() * 25];
    // if map.len() is 10000, axis_len is 100
    let axis_len = (map.len() as f64).sqrt() as usize;
    for y in 0..axis_len {
        for x in 0..axis_len {
            // Set up our array offset as usual
            let y = y * axis_len;
            for z_x in 0..5 {
                for z_y in 0..5 {
                    // With a flattened array, x offsets don't move very far.  y offsets however...
                    let tile_offset = (z_y * map.len() * 5) + z_x * axis_len;

                    let mut value = map[y + x] + z_x as u32 + z_y as u32;
                    if value > 9 {
                        value = (value + 1) % 10;
                    }

                    // Each y position now has to move 5x as far to go down a line
                    let y = y * 5;

                    map_but_more_big[tile_offset + y + x] = value;
                }
            }
        }
    }

    println!("Part 2: {}", find_best_path(&map_but_more_big));

    Ok(())
}
