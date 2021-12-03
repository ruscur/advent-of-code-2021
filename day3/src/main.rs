use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn parse(buf: &mut BufReader<File>) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let mut report: Vec<u16> = vec![];

    // as before, we're treating parsing separately from solution logic
    for line in buf.lines() {
        report.push(u16::from_str_radix(&line?, 2)?);
    }
    Ok(report)
}

fn get_oxygen_generator_rating(reports: &Vec<u16>, diag_len: u16) -> u16 {
    let mut filter: Vec<u16> = reports.clone();
    for x in (0..diag_len).rev() {
        let ones_count = filter.iter().filter(|diag| (*diag & (1 << x) > 0)).count();
        let ones_common = if ones_count * 2 >= filter.len() { 1 } else { 0 };
        filter = filter.into_iter().filter(|diag| (*diag & (1 << x)) == (ones_common << x)).collect();

        if filter.len() == 1 { return *filter.get(0).unwrap() }
    }
    0
}

fn get_co2_scrubber_rating(reports: &Vec<u16>, diag_len: u16) -> u16 {
    let mut filter: Vec<u16> = reports.clone();
    for x in (0..diag_len).rev() {
        let ones_count = filter.iter().filter(|diag| (*diag & (1 << x) > 0)).count();
        let ones_uncommon = if ones_count * 2 >= filter.len() { 0 } else { 1 };

        filter = filter.into_iter().filter(|diag| (*diag & (1 << x)) == (ones_uncommon << x)).collect();

        if filter.len() == 1 { return *filter.get(0).unwrap() }
    }
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let report = parse(&mut reader)?;
    let diag_len: u16 = 12;
    let report_len = report.len();

    assert!(report_len > 0);

    let most_common: String = (0..diag_len)
        .rev() // most significant to least significant: 15, 14, 13...
        .map(|x| {
            let ones_count = report.iter().filter(|diag| (*diag & (1 << x)) > 0).count();
            dbg!(ones_count);
            if ones_count * 2 > report_len {
                '1'
            } else {
                '0'
            }
        })
        .collect();

    dbg!(&most_common, report_len);

    let gamma_rate = u16::from_str_radix(&most_common, 2)?;
    let epsilon_rate = !gamma_rate & 0xfff;
    let power_consumption = (gamma_rate as u64) * (epsilon_rate as u64);

    dbg!(gamma_rate, epsilon_rate, power_consumption);

    let oxygen_generator_rating: u16 = get_oxygen_generator_rating(&report, diag_len);
    let co2_scrubber_rating: u16 = get_co2_scrubber_rating(&report, diag_len);
    let life_support_rating = (oxygen_generator_rating as u64) * (co2_scrubber_rating as u64);

    dbg!(oxygen_generator_rating, co2_scrubber_rating, life_support_rating);

    Ok(())
}
