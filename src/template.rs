use anyhow::Result;
use std::fs;

fn read_input(inputfile: &str) -> Result<(Vec<i32>, Vec<Vec<i32>>)> {
    let data = fs::read_to_string(inputfile)?;
    for line in data.lines() {
    }
    Ok(...
}

fn part1(inputfile: &str) -> Result<i32> {
    let ... = read_input(inputfile)?;
    Ok(...)
}

fn part2(inputfile: &str) -> Result<i32> {
    let ... = read_input(inputfile)?;
    Ok(...)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day..._test.txt").unwrap(), ...);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day..._test.txt").unwrap(), ...);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
