use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let digits: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    Ok(digits)
}

// For a given position, this struct tracks what's adjacent to it,
// and if we've tried going there before.
// If there's a 9 in a direction, checked_direction = true.
#[derive(Debug, Copy, Clone)]
struct Adjacent {
    up: u8,
    down: u8,
    left: u8,
    right: u8,
    checked_up: bool,
    checked_down: bool,
    checked_left: bool,
    checked_right: bool,
}

fn get_adjacent(map: &Vec<Vec<u8>>, x: usize, y: usize, x_len: usize, y_len: usize) -> Adjacent {
    let mut adjacent = Adjacent {
        up: 9,
        down: 9,
        left: 9,
        right: 9,
        checked_up: true,
        checked_down: true,
        checked_left: true,
        checked_right: true,
    };

    if x != 0 {
        adjacent.left = map[y][x - 1];
        if adjacent.left != 9 {
            adjacent.checked_left = false;
        }
    }

    if x != x_len - 1 {
        adjacent.right = map[y][x + 1];
        if adjacent.right != 9 {
            adjacent.checked_right = false;
        }
    }

    if y != 0 {
        adjacent.up = map[y - 1][x];
        if adjacent.up != 9 {
            adjacent.checked_up = false;
        }
    }

    if y != y_len - 1 {
        adjacent.down = map[y + 1][x];
        if adjacent.down != 9 {
            adjacent.checked_down = false;
        }
    }

    adjacent
}

fn is_smaller_than_adjacent(x: u8, adjacent: Adjacent) -> bool {
    x < adjacent.up && x < adjacent.down && x < adjacent.left && x < adjacent.right
}

fn build_basin(
    map: &Vec<Vec<u8>>,
    x_pos: usize,
    y_pos: usize,
    x_len: usize,
    y_len: usize,
) -> Vec<(usize, usize)> {
    // Our starting position is the low point of the basin
    let mut x = x_pos;
    let mut y = y_pos;
    let mut adjacent = get_adjacent(&map, x, y, x_len, y_len);
    // Maps a (x, y) coordinate to its adjacency data
    let mut positions: BTreeMap<(usize, usize), Adjacent> = BTreeMap::new();

    positions.insert((x, y), adjacent);

    // This is a very chaotic (read: bad) implementation of a search.
    // We keep going places until we're in a corner.  Then, we check everywhere
    // we've been so far for a yet-to-be-explored direction, and jump to that
    // position.  Once we're out of places to go, we quit.
    //
    // I'm definitely not happy with this, but also I've somehow never written
    // anything like this before, and it's a pretty common thing that comes up
    // in puzzles and interview questions and stuff.  Definitely gonna check
    // out how others did it after this.  Maybe I should try some recursion.
    loop {
        if !adjacent.checked_up {
            adjacent.checked_up = true;
            positions.insert((x, y), adjacent);
            y -= 1;
        } else if !adjacent.checked_left {
            adjacent.checked_left = true;
            positions.insert((x, y), adjacent);
            x -= 1;
        } else if !adjacent.checked_down {
            adjacent.checked_down = true;
            positions.insert((x, y), adjacent);
            y += 1;
        } else if !adjacent.checked_right {
            adjacent.checked_right = true;
            positions.insert((x, y), adjacent);
            x += 1;
        } else {
            // Have we been anywhere that still has some exploration to do?
            if let Some(unexplored) = positions.iter().find(|&((_, _), adj)| {
                adj.checked_down == false
                    || adj.checked_up == false
                    || adj.checked_left == false
                    || adj.checked_right == false
            }) {
                // Jump to that position and keep going
                x = unexplored.0 .0;
                y = unexplored.0 .1;
                adjacent = *unexplored.1;
                continue;
            } else {
                break;
            }
        }

        // get_adjacent() would wipe our exploration progress
        if positions.contains_key(&(x, y)) {
            adjacent = *positions.get(&(x, y)).unwrap()
        } else {
            adjacent = get_adjacent(&map, x, y, x_len, y_len);
            positions.insert((x, y), adjacent);
        }
    }

    positions.keys().map(|&x| x).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let heights = parse(lines?)?;
    let y_len = heights.len();
    let x_len = heights.iter().next().unwrap().len();

    // value, x_pos, y_pos
    let low_points: Vec<(u8, usize, usize)> = heights
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, val)| {
                    is_smaller_than_adjacent(*val as u8, get_adjacent(&heights, x, y, x_len, y_len))
                })
                .map(|(x, val)| (*val as u8, x, y))
                .collect::<Vec<(u8, usize, usize)>>()
        })
        .flatten()
        .collect();

    println!(
        "Part 1: {}",
        low_points.iter().map(|&x| (x.0 + 1) as u64).sum::<u64>()
    );

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|point| build_basin(&heights, point.1, point.2, x_len, y_len).len())
        .collect();

    // lowest to highest
    basin_sizes.sort();
    // highest to lowest
    basin_sizes.reverse();

    println!(
        "Part 2: {}",
        basin_sizes.iter().take(3).fold(1, |size, acc| acc * size)
    );

    Ok(())
}
