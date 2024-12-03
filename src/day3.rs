use anyhow::Result;
use regex::Regex;
use std::fs;

fn part1(inputfile: &str) -> Result<i32> {
    let data = fs::read_to_string(inputfile)?;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let mut total: i32 = 0;
    for cap in re.captures_iter(&data) {
        let (_, [a, b]) = cap.extract();
        let a: i32 = a.parse()?;
        let b: i32 = b.parse()?;
        total += a * b;
    }
    Ok(total)
}

fn part2(inputfile: &str) -> Result<i32> {
    let data = fs::read_to_string(inputfile)?;
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))")?;
    let mut doing = true;
    let mut total = 0;
    for cap in re.captures_iter(&data) {
        if cap.get(4).is_some() {
            doing = true;
        } else if cap.get(5).is_some() {
            doing = false;
        } else if cap.get(1).is_some() && doing {
            let a: i32 = cap.get(2).unwrap().as_str().parse()?;
            let b: i32 = cap.get(3).unwrap().as_str().parse()?;
            total += a * b;
        }
    }
    Ok(total)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day3_test_part1.txt").unwrap(), 161);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day3_test_part2.txt").unwrap(), 48);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
