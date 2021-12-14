use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// Map character pairs to the two pairs they'll split into.
fn parse_insertions(
    lines: Vec<String>,
) -> Result<BTreeMap<(char, char), ((char, char), (char, char))>, Box<dyn std::error::Error>> {
    let insertions: BTreeMap<(char, char), ((char, char), (char, char))> = lines
        .iter()
        .map(|l| l.replace(" -> ", "").chars().collect::<Vec<char>>())
        .map(|chars| {
            (
                (chars[0], chars[1]),
                ((chars[0], chars[2]), (chars[2], chars[1])),
            )
        })
        .collect();

    Ok(insertions)
}

fn process_insertions(
    pairs: BTreeMap<(char, char), usize>,
    map: &BTreeMap<(char, char), ((char, char), (char, char))>,
) -> BTreeMap<(char, char), usize> {
    let mut update = pairs.clone();

    for (pair, count) in pairs.iter() {
        // If KV occurs 3 times, and splits into KP and PV...
        let (first, last) = map[pair];
        // there will be 3 new KPs,
        *update.entry(first).or_default() += count;
        // there will be 3 new PVs,
        *update.entry(last).or_default() += count;
        // and no KVs, because we just split them all up.
        // We can't set this to 0, because that would destroy
        // any new instances of this pair generated this cycle.
        *update.entry(*pair).or_default() -= count;
    }

    update
}

fn count_letters(
    pair_count: &BTreeMap<(char, char), usize>,
    start: char,
    end: char,
) -> BTreeMap<char, usize> {
    let mut char_count: BTreeMap<char, usize> = BTreeMap::new();

    for (pair, count) in pair_count {
        *char_count.entry(pair.0).or_default() += count;
        *char_count.entry(pair.1).or_default() += count;
    }

    // there's two pair occurrences for every one character...
    char_count = char_count
        .iter()
        .map(|(c, count)| (*c, count / 2))
        .collect();

    // ...except for the first and last character.
    *char_count.entry(start).or_default() += 1;
    *char_count.entry(end).or_default() += 1;

    char_count
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
    let template = iter.next().unwrap()[0].to_owned();

    let start_letter = template.chars().next().unwrap();
    let end_letter = template.chars().next_back().unwrap();

    let insertions = parse_insertions(iter.next().unwrap().to_vec())?;

    let mut initial_count: BTreeMap<(char, char), usize> = BTreeMap::new();

    for pair in template
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(2)
    {
        *initial_count.entry((pair[0], pair[1])).or_default() += 1;
    }

    let tenth = (0..10).fold(initial_count, |count, _| {
        process_insertions(count, &insertions)
    });

    let tenth_char_count = count_letters(&tenth, start_letter, end_letter);

    println!(
        "Part 1: {}",
        tenth_char_count.values().max().unwrap() - tenth_char_count.values().min().unwrap()
    );

    let fortieth = (10..40).fold(tenth, |count, _| process_insertions(count, &insertions));

    let fortieth_char_count = count_letters(&fortieth, start_letter, end_letter);

    println!(
        "Part 2: {}",
        fortieth_char_count.values().max().unwrap() - fortieth_char_count.values().min().unwrap()
    );

    Ok(())
}
