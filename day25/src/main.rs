use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(lines: Vec<String>) -> Result<Vec<Vec<Option<bool>>>, Box<dyn std::error::Error>> {
    let cucumbers = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    'v' => Some(true),
                    '>' => Some(false),
                    _ => None,
                })
                .collect::<Vec<Option<bool>>>()
        })
        .collect::<Vec<Vec<Option<bool>>>>();

    Ok(cucumbers)
}

fn step(map: &Vec<Vec<Option<bool>>>) -> Vec<Vec<Option<bool>>> {
    let mut updated = map.clone();
    let y_len = map.len();
    let x_len = map[0].len();

    // handle the easties
    for y in 0..y_len {
        for x in 0..x_len {
            if let Some(down) = map[y][x] {
                if down {
                    // moving down
                } else {
                    // moving right
                    if map[y][(x + 1) % x_len].is_none() {
                        updated[y][(x + 1) % x_len] = Some(down);
                        updated[y][x] = None;
                    }
                }
            }
        }
    }

    let map = updated;
    let mut updated = map.clone();
    // handle the southies
    for y in 0..y_len {
        for x in 0..x_len {
            if let Some(down) = map[y][x] {
                if down {
                    // moving down
                    if map[(y + 1) % y_len][x].is_none() {
                        updated[(y + 1) % y_len][x] = Some(down);
                        updated[y][x] = None;
                    }
                } else {
                    // moving right
                }
            }
        }
    }

    updated
}

fn find_stop_step(start: &Vec<Vec<Option<bool>>>) -> usize {
    let mut i = 1;

    let mut prev = start.clone();
    loop {
        let next = step(&prev);

        if prev == next {
            return i;
        }

        prev = next;
        i += 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();

    let mut cucumbers = parse(lines?)?;

    println!("Part 1: {}", find_stop_step(&cucumbers));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    const EXAMPLE1_1: &str = "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v";

    const EXAMPLE1_2: &str = ">.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv.";

    const EXAMPLE1_3: &str = "v>v.v>.>v.
v...>>.v.v
>vv>.>v>..
>>v>v.>.v>
..>....v..
.>.>v>v..v
..v..v>vv>
v.v..>>v..
.v>....v..";

    const EXAMPLE1_4: &str = "v>..v.>>..
v.v.>.>.v.
>vv.>>.v>v
>>.>..v>.>
..v>v...v.
..>>.>vv..
>.v.vv>v.v
.....>>vv.
vvv>...v..";

    const EXAMPLE1_5: &str = "vv>...>v>.
v.v.v>.>v.
>.v.>.>.>v
>v>.>..v>>
..v>v.v...
..>.>>vvv.
.>...v>v..
..v.v>>v.v
v.v.>...v.";

    const EXAMPLE1_10: &str = "..>..>>vv.
v.....>>.v
..v.v>>>v>
v>.>v.>>>.
..v>v.vv.v
.v.>>>.v..
v.v..>v>..
..v...>v.>
.vv..v>vv.";

    const EXAMPLE1_20: &str = "v>.....>>.
>vv>.....v
.>v>v.vv>>
v>>>v.>v.>
....vv>v..
.v.>>>vvv.
..v..>>vv.
v.v...>>.v
..v.....v>";

    const EXAMPLE1_30: &str = ".vv.v..>>>
v>...v...>
>.v>.>vv.>
>v>.>.>v.>
.>..v.vv..
..v>..>>v.
....v>..>v
v.v...>vv>
v.v...>vvv";

    const EXAMPLE1_40: &str = ">>v>v..v..
..>>v..vv.
..>>>v.>.v
..>>>>vvv>
v.....>...
v.v...>v>>
>vv.....v>
.>v...v.>v
vvv.v..v.>";

    const EXAMPLE1_50: &str = "..>>v>vv.v
..v.>>vv..
v.>>v>>v..
..>>>>>vv.
vvv....>vv
..v....>>>
v>.......>
.vv>....v>
.>v.vv.v..";

    const EXAMPLE1_55: &str = "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv...>..>
>vv.....>.
.>v.vv.v..";

    const EXAMPLE1_56: &str = "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv....>.>
>vv......>
.>v.vv.v..";

    const EXAMPLE1_57: &str = "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v..";

    const EXAMPLE1_58: &str = "..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v..";

    #[test]
    fn basic_test() {
        let start = "...>>>>>...".to_string();
        let step1_str = "...>>>>.>..".to_string();
        let step2_str = "...>>>.>.>.".to_string();
        let cucumbers = parse(vec![start]).unwrap();

        let step1 = step(&cucumbers);
        let step2 = step(&step1);
        assert!(step1 == parse(vec![step1_str]).unwrap());
        assert!(step2 == parse(vec![step2_str]).unwrap());
    }

    #[test]
    fn move_both_test() {
        let example = "..........
.>v....v..
.......>..
.........."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let step1_str = "..........
.>........
..v....v>.
.........."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let cucumbers = parse(example).unwrap();

        let step1 = step(&cucumbers);
        assert!(step1 == parse(step1_str).unwrap());
    }

    #[test]
    fn wrap_test() {
        let example = "...>...
.......
......>
v.....>
......>
.......
..vvv.."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let step1_str = "..vv>..
.......
>......
v.....>
>......
.......
....v.."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let step2_str = "....v>.
..vv...
.>.....
......>
v>.....
.......
......."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let step3_str = "......>
..v.v..
..>v...
>......
..>....
v......
......."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let step4_str = ">......
..v....
..>.v..
.>.v...
...>...
.......
v......"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let cucumbers = parse(example).unwrap();

        let step1 = step(&cucumbers);
        let step2 = step(&step1);
        let step3 = step(&step2);
        let step4 = step(&step3);
        assert!(step1 == parse(step1_str).unwrap());
        assert!(step2 == parse(step2_str).unwrap());
        assert!(step3 == parse(step3_str).unwrap());
        assert!(step4 == parse(step4_str).unwrap());
    }

    #[test]
    fn find_stop_test() {
        let cucumbers = parse(
            EXAMPLE1
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step1 = parse(
            EXAMPLE1_1
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step2 = parse(
            EXAMPLE1_2
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step3 = parse(
            EXAMPLE1_3
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step4 = parse(
            EXAMPLE1_4
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step5 = parse(
            EXAMPLE1_5
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step10 = parse(
            EXAMPLE1_10
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step20 = parse(
            EXAMPLE1_20
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step30 = parse(
            EXAMPLE1_30
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step40 = parse(
            EXAMPLE1_40
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step50 = parse(
            EXAMPLE1_50
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step55 = parse(
            EXAMPLE1_55
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step56 = parse(
            EXAMPLE1_56
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step57 = parse(
            EXAMPLE1_57
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();
        let step58 = parse(
            EXAMPLE1_58
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        )
        .unwrap();

        let mut i = 1;
        let mut prev = cucumbers.clone();
        loop {
            let next = step(&prev);
            match i {
                1 => assert!(next == step1),
                2 => assert!(next == step2),
                3 => assert!(next == step3),
                4 => assert!(next == step4),
                5 => assert!(next == step5),
                10 => assert!(next == step10),
                20 => assert!(next == step20),
                30 => assert!(next == step30),
                40 => assert!(next == step40),
                50 => assert!(next == step50),
                55 => assert!(next == step55),
                56 => assert!(next == step56),
                57 => assert!(next == step57),
                58 => assert!(next == step58),
                _ => {}
            }
            if next == prev {
                break;
            }
            prev = next;
            i += 1;
        }
        assert!(i == 58);
        assert!(find_stop_step(&cucumbers) == 58);
    }
}
