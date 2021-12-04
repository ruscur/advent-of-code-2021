use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

fn parse(lines: &mut Lines<BufReader<File>>) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut bingo_boards: Vec<Vec<u8>> = vec![];

    let mut board: Vec<u8> = vec![];

    for board_lines in lines.skip(1) {
        let board_lines = board_lines?;
        if board_lines.is_empty() {
            bingo_boards.push(board);
            board = vec![];
        } else {
            board_lines
                .split_whitespace()
                .for_each(|x| board.push(x.parse::<u8>().unwrap()));
        }
    }
    Ok(bingo_boards)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap()?;

    let draw_order: Vec<u8> = first_line
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut boards = parse(&mut lines)?;
    let board_count = boards.len();
    let mut matches_list: Vec<[bool; 25]> = vec![[false; 25]; boards.len()];
    assert!(board_count > 0);

    for draw in draw_order {
        let mut new_winners: Vec<usize> = vec![];
        for (j, board) in boards.iter().enumerate() {
            for (i, x) in board.iter().enumerate() {
                if x == &draw {
                    let mut matches = matches_list.get(j).unwrap().clone();
                    matches[i] = true;
                    matches_list.push(matches);
                    matches_list.swap_remove(j);

                    let row = i - (i % 5);

                    if !((matches[(i + 5) % 25]
                        && matches[(i + 10) % 25]
                        && matches[(i + 15) % 25]
                        && matches[(i + 20) % 25])
                        || (matches[row]
                            && matches[row + 1]
                            && matches[row + 2]
                            && matches[row + 3]
                            && matches[row + 4]))
                    {
                        continue;
                    }

                    let winner_count = board_count - boards.len() + new_winners.len();
                    if winner_count == 0 || winner_count == board_count - 1 {
                        let unmarked_sum: u64 = board
                            .iter()
                            .enumerate()
                            .filter(|&(i, _)| !matches[i])
                            .fold(0, |acc, (_, x)| acc + (*x as u64));

                        let total = unmarked_sum * (draw as u64);
                        if winner_count == 0 {
                            println!("Part 1: {}", total);
                        } else {
                            println!("Part 2: {}", total);
                        }
                    }
                    new_winners.push(j);
                    break;
                }
            }
        }

        // We're going to remove entries from the vec, so we need to remove from the right
        // to preserve existing indices.
        new_winners.sort();
        new_winners.reverse();
        for x in new_winners {
            boards.remove(x);
            matches_list.remove(x);
        }
    }

    Ok(())
}
