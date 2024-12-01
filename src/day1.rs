use std::fs;
use std::iter::zip;
use anyhow::Result;

struct PairOfLists {
    left: Vec::<i32>,
    right: Vec::<i32>
}

fn read_data(inputfile: &str, sort: bool) -> Result<PairOfLists> {
    let data = fs::read_to_string(inputfile)?;
    let mut lists = PairOfLists { left: Vec::<i32>::new(), right: Vec::<i32>::new() };
    for line in data.lines() {
        let fields:Vec<&str> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 2);
        lists.left.push(fields[0].parse()?);
        lists.right.push(fields[1].parse()?);
    }
    if sort {
        lists.left.sort();
        lists.right.sort();
    }
    Ok(lists)
}
    
fn part1(inputfile: &str) -> Result<i32> {
    let lists = read_data(inputfile, true)?;
    let mut sum_diff:i32 = 0;
    for (a, b) in zip(&lists.left, &lists.right) {
        // println!("{a} {b}");
        sum_diff += (b - a).abs();
    }
    Ok(sum_diff)
}

fn part2(inputfile: &str) -> Result<i32> {
    let lists = read_data(inputfile, false)?;
    let mut similarity: i32 = 0;
    for a in lists.left {
        let count = lists.right.iter().filter(|&b| *b == a).count();
        similarity += a * count as i32;
    }
    Ok(similarity)
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day1_test.txt").unwrap(), 11);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day1_test.txt").unwrap(), 31);
}
