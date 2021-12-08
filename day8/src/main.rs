use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn char_to_digit(ch: char) -> u8 {
    match ch {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("whoops {}", ch),
    }
}

fn count_digits(digits: Vec<u8>) -> u8 {
    digits.iter().filter(|&x| *x == 1).count() as u8
}

fn parse(
    lines: Vec<String>,
) -> Result<Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>, Box<dyn std::error::Error>> {
    let mut entries = vec![];

    for line in lines {
        let (patterns, output) = line.split_once("|").unwrap();

        let patterns: Vec<Vec<u8>> = patterns
            .split_whitespace()
            .map(|x| {
                let mut segments = [0u8; 7];
                x.chars()
                    .map(char_to_digit)
                    .for_each(|x| segments[x as usize] = 1);
                segments.to_vec()
            })
            .collect();
        let output: Vec<Vec<u8>> = output
            .split_whitespace()
            .map(|x| {
                let mut segments = [0u8; 7];
                x.chars()
                    .map(char_to_digit)
                    .for_each(|x| segments[x as usize] = 1);
                segments.to_vec()
            })
            .collect();
        entries.push((patterns, output));
    }

    Ok(entries)
}

const SEGMENT_LENGTHS: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
const SEGMENTS: [[u8; 7]; 10] = [
    [1, 1, 1, 0, 1, 1, 1], // 0
    [0, 0, 1, 0, 0, 1, 0], // 1
    [1, 0, 1, 1, 1, 0, 1], // 2
    [1, 0, 1, 1, 0, 1, 1], // 3
    [0, 1, 1, 1, 0, 1, 0], // 4
    [1, 1, 0, 1, 0, 1, 1], // 5
    [1, 1, 0, 1, 1, 1, 1], // 6
    [1, 0, 1, 0, 0, 1, 0], // 7
    [1, 1, 1, 1, 1, 1, 1], // 8
    [1, 1, 1, 1, 0, 1, 1], // 9
];

fn deduce_segments(pattern: Vec<Vec<u8>>) -> Vec<u8> {
    let mut digits = vec![vec![0; 7]; 10];
    // a == 0, b == 1 etc
    let mut segments = [255u8; 7];

    let mut c_or_f = vec![];
    let mut b_or_d = vec![];

    // Solve the easy ones first.
    [1, 4, 7, 8].iter().for_each(|x| {
        digits[*x as usize] = pattern
            .iter()
            .filter(|y| {
                y.iter().filter(|&z| *z == 1).count() == SEGMENT_LENGTHS[*x as usize] as usize
            })
            .next()
            .unwrap()
            .to_vec()
    });

    (0..7).for_each(|x| {
        if digits[1][x as usize] == 1 {
            c_or_f.push(x);
        }
    });

    // Now let's figure out what some of the segments are.
    // 7 is just 1 with segment 'a' as an extra.
    segments[0] = (0..10)
        .find(|&x| (digits[7][x] ^ digits[1][x]) == 1)
        .unwrap() as u8;
    // 4 has 'b' and 'd' over 1.
    (0..7).for_each(|x| {
        if (digits[4][x] ^ digits[1][x]) == 1 {
            b_or_d.push(x as u8)
        }
    });

    // Of the three with 6 segments (0, 6, 9), 0 misses 'd', 6 misses 'c', 9 misses 'e'.
    // So the digit containing all of (b, c, d, f) must be 9.
    digits[9] = pattern
        .iter()
        .filter(|&x| count_digits(x.to_vec()) == 6)
        .find(|x| {
            (x[c_or_f[0] as usize]
                & x[c_or_f[1] as usize]
                & x[b_or_d[0] as usize]
                & x[b_or_d[1] as usize])
                == 1
        })
        .unwrap()
        .to_vec();
    // Since we have 9, we can find 'e'.
    segments[4] = (0..7).find(|&x| digits[9][x as usize] == 0).unwrap();

    // The digit missing one of (b, d) must be 0
    digits[0] = pattern
        .iter()
        .filter(|&x| count_digits(x.to_vec()) == 6)
        .find(|x| (x[b_or_d[0] as usize] ^ x[b_or_d[1] as usize]) == 1)
        .unwrap()
        .to_vec();
    // Now we can figure out 'b' and 'd'
    segments[3] = (0..7).find(|&x| digits[0][x as usize] == 0).unwrap();
    segments[1] = if b_or_d[0] == segments[3] {
        b_or_d[1]
    } else {
        b_or_d[0]
    };

    // The digit missing one of (c, f) must be 6
    digits[6] = pattern
        .iter()
        .filter(|&x| count_digits(x.to_vec()) == 6)
        .find(|x| (x[c_or_f[0] as usize] ^ x[c_or_f[1] as usize]) == 1)
        .unwrap()
        .to_vec();
    // Now we can figure out 'c' and 'f'
    segments[2] = (0..7).find(|&x| digits[6][x as usize] == 0).unwrap();
    segments[5] = if c_or_f[0] == segments[2] {
        c_or_f[1]
    } else {
        c_or_f[0]
    };

    // We have everything except 'g' now, so it's the odd one out.
    segments[6] = (0..7).find(|x| !segments.contains(x)).unwrap();

    segments.to_vec()
}

fn translate_output(output: Vec<Vec<u8>>, mapping: Vec<u8>) -> Vec<u8> {
    let translated_segments: Vec<Vec<u8>> = SEGMENTS
        .map(|x| {
            let mut translation = [0u8; 7];
            (0..7).for_each(|n| {
                if x[n] == 1 {
                    translation[mapping[n] as usize] = 1
                }
            });
            translation.to_vec()
        })
        .to_vec();

    output
        .iter()
        .map(|num| {
            translated_segments
                .iter()
                .enumerate()
                .find(|(_, x)| *x == num)
                .unwrap()
                .0 as u8
        })
        .collect()
}

fn digits_to_value(digits: Vec<u8>) -> u16 {
    // Could just do digits[0]*1000 + digits[1]*100 etc but nah
    digits
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u16>()
        .unwrap()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let data = parse(lines?)?;

    let values: Vec<Vec<u8>> = data
        .iter()
        .map(|(pattern, output)| {
            translate_output(output.to_vec(), deduce_segments(pattern.to_vec()))
        })
        .collect();

    println!(
        "Part 1: {}",
        values
            .iter()
            .map(|x| x
                .iter()
                .filter(|&y| *y == 1 || *y == 4 || *y == 7 || *y == 8)
                .count())
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        values
            .iter()
            .map(|x| digits_to_value(x.to_vec()) as u64)
            .sum::<u64>()
    );

    Ok(())
}
