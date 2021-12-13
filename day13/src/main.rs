use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn fold(&self, page: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        match self {
            Fold::X(pos) => fold_left(&page, *pos),
            Fold::Y(pos) => fold_up(&page, *pos),
        }
    }
}

fn parse_coords(
    lines: Vec<String>,
    x: usize,
    y: usize,
) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    // Why are my 2d arrays always [y][x] indexed
    let mut page: Vec<Vec<bool>> = vec![vec![false; x]; y];

    lines
        .iter()
        .filter_map(|l| l.split_once(","))
        .filter_map(|(x, y)| Some((x.parse::<usize>().ok()?, y.parse::<usize>().ok()?)))
        .for_each(|(x, y)| page[y][x] = true);

    Ok(page)
}

fn parse_folds(lines: Vec<String>) -> Result<Vec<Fold>, Box<dyn std::error::Error>> {
    Ok(lines
        .iter()
        .filter_map(|l| l.strip_prefix("fold along "))
        .filter_map(|l| l.split_once("="))
        .filter_map(|(axis, val)| Some((axis, val.parse::<usize>().ok()?)))
        .map(|(axis, val)| {
            if axis == "x" {
                Fold::X(val)
            } else {
                Fold::Y(val)
            }
        })
        .collect())
}

fn fold_left(page: &Vec<Vec<bool>>, x_size: usize) -> Vec<Vec<bool>> {
    let y_size = page.len();
    let mut new_page = vec![vec![false; x_size]; y_size];

    for y in 0..y_size {
        for x in 0..x_size {
            new_page[y][x] = page[y][x] || page[y][(x_size * 2) - x];
        }
    }

    new_page
}

fn fold_up(page: &Vec<Vec<bool>>, y_size: usize) -> Vec<Vec<bool>> {
    let x_size = page[0].len();
    let mut new_page = vec![vec![false; x_size]; y_size];

    for y in 0..y_size {
        for x in 0..x_size {
            new_page[y][x] = page[y][x] || page[y_size * 2 - y][x];
        }
    }

    new_page
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
    let mut iter = lines.split(|l| l.len() == 0);
    let coords = iter.next().unwrap();

    let folds = parse_folds(iter.next().unwrap().to_vec())?;
    let max_x = folds
        .iter()
        .find_map(|fold| match fold {
            Fold::X(val) => Some(val),
            _ => None,
        })
        .unwrap()
        * 2
        + 1;
    let max_y = folds
        .iter()
        .find_map(|fold| match fold {
            Fold::Y(val) => Some(val),
            _ => None,
        })
        .unwrap()
        * 2
        + 1;

    let page = parse_coords(coords.to_vec(), max_x, max_y)?;

    println!(
        "Part 1: {}",
        folds[0]
            .fold(&page)
            .iter()
            .flatten()
            .filter(|&x| *x)
            .count()
    );

    // This is so amusing to me I'm not changing the variable names.
    let page = folds.iter().fold(page, |page, fold| fold.fold(&page));

    println!("Part 2:");
    for y in page {
        for x in y {
            print!("{}", if x { "#" } else { "." });
        }
        print!("\n");
    }

    Ok(())
}
