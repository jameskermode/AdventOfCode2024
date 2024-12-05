use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering::*;
use std::fs;

fn parts_12(inputfile: &str) -> Result<(i32, i32)> {
    let data = fs::read_to_string(inputfile)?;
    let parts: Vec<_> = data.split("\n\n").collect();
    assert!(parts.len() == 2);

    let re = Regex::new(r"([0-9]+)\|([0-9]+)")?;
    let mut rules: Vec<(usize, usize)> = vec![];
    for cap in re.captures_iter(parts[0]) {
        let (_, [first, second]) = cap.extract();
        let first: usize = first.parse().unwrap();
        let second: usize = second.parse().unwrap();
        rules.push((first, second));
    }

    let updates: Vec<Vec<usize>> = parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|field| field.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut part1_total = 0;
    let mut part2_total = 0;
    for update in &updates {
        let mut new_update: Vec<usize> = update.clone();
        new_update.sort_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Less
            } else if rules.contains(&(*b, *a)) {
                Greater
            } else {
                Equal
            }
        });
        let valid = new_update == *update;
        println!("{valid} {update:?} {new_update:?}");
        if valid {
            part1_total += update[update.len() / 2];
        } else {
            part2_total += new_update[new_update.len() / 2];
        }
    }
    Ok((part1_total as i32, part2_total as i32))
}

#[test]
fn test_parts() {
    assert_eq!(parts_12("./input/day5_test.txt").unwrap(), (143, 123));
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok(parts_12(inputfile)?)
}
