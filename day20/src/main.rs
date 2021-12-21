use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

#[cfg(test)]
fn vec_to_str(pixels: &Vec<bool>) -> String {
    pixels
        .iter()
        .map(|p| if *p { '#' } else { '.' })
        .collect::<String>()
}

fn str_to_vec(pixels: &String) -> Vec<bool> {
    pixels
        .chars()
        .map(|c| if c == '#' { true } else { false })
        .collect()
}

#[cfg(test)]
fn print_image(image: &Vec<Vec<bool>>) {
    for y in image {
        println!("{}", vec_to_str(y));
    }
}

fn get_square_value(image: &Vec<Vec<bool>>, x: usize, y: usize) -> Result<usize, ParseIntError> {
    let mut value = vec![image[0][0]; 9];
    let x_len = image[0].len();
    let y_len = image.len();

    if x > 0 && y > 0 {
        value[0] = image[y - 1][x - 1];
    }

    if y > 0 {
        value[1] = image[y - 1][x];
    }

    if y > 0 && x < x_len - 1 {
        value[2] = image[y - 1][x + 1];
    }

    if x > 0 {
        value[3] = image[y][x - 1];
    }

    value[4] = image[y][x];

    if x < x_len - 1 {
        value[5] = image[y][x + 1];
    }

    if y < y_len - 1 && x > 0 {
        value[6] = image[y + 1][x - 1];
    }

    if y < y_len - 1 {
        value[7] = image[y + 1][x];
    }

    if y < y_len - 1 && x < x_len - 1 {
        value[8] = image[y + 1][x + 1];
    }

    usize::from_str_radix(
        &value
            .iter()
            .map(|p| if *p { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
}

fn parse(lines: Vec<String>) -> Result<(Vec<bool>, Vec<Vec<bool>>), Box<dyn std::error::Error>> {
    let mut page: Vec<Vec<bool>> = vec![];
    let mut algo = None;

    for line in lines {
        if algo.is_none() {
            algo = Some(str_to_vec(&line));
        } else if line.is_empty() {
            continue;
        } else {
            page.push(str_to_vec(&line));
        }
    }

    Ok((algo.unwrap(), page))
}

fn grow_image(image: &Vec<Vec<bool>>, inf: bool) -> Vec<Vec<bool>> {
    let y_size = image.len();
    let x_size = image[0].len();
    let mut grown = vec![vec![inf; x_size + 4]; y_size + 4];

    for y in 0..y_size {
        for x in 0..x_size {
            grown[y + 2][x + 2] = image[y][x];
        }
    }

    grown
}

fn update_image(
    old: &Vec<Vec<bool>>,
    algo: &Vec<bool>,
) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    let grown = grow_image(old, old[0][0]);
    let mut updated = grow_image(old, old[0][0]);

    for y in 0..grown.len() {
        for x in 0..grown[0].len() {
            updated[y][x] = algo[get_square_value(&grown, x, y)?];
        }
    }

    Ok(updated)
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

    let (algo, image) = parse(lines)?;

    let grown = grow_image(&image, false);
    let first = update_image(&grown, &algo)?;
    let second = update_image(&first, &algo)?;

    println!(
        "Part 1: {}",
        second.iter().flatten().filter(|&p| *p).count()
    );

    let fifty = (0..50).fold(grown, |prev, _| update_image(&prev, &algo).unwrap());

    println!("Part 2: {}", fifty.iter().flatten().filter(|&p| *p).count());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn index_test() {
        let (algo, image) = parse(
            INPUT
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();

        let index = get_square_value(&image, 2, 2).unwrap();

        assert!(index == 34);
        assert!(algo[34]);
    }

    #[test]
    fn grow_test() {
        let (_algo, image) = parse(
            INPUT
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();

        let grown_image = grow_image(&image, false);

        assert!(grown_image.len() == 9);
        assert!(grown_image[0].len() == 9);
        assert!(get_square_value(&grown_image, 4, 4).unwrap() == 34);
    }

    #[test]
    fn update_test() {
        let (algo, image) = parse(
            INPUT
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();

        let grown = grow_image(&image, false);
        print_image(&grown);
        let updated = update_image(&grown, &algo).unwrap();
        print_image(&updated);
        let updated_twice = update_image(&updated, &algo).unwrap();
        print_image(&updated_twice);
        assert!(updated_twice.iter().flatten().filter(|&p| *p).count() == 35);
    }
}
