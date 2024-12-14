use anyhow::Result;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Record {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}

fn read_input(inputfile: &str) -> Result<Vec<Record>> {
    let data = fs::read_to_string(inputfile)?;

    let re = Regex::new(
        r"(?ms)Button A: X\+(?<AX>[0-9]+), Y\+(?<AY>[0-9]+).
Button B: X\+(?<BX>[0-9]+), Y\+(?<BY>[0-9]+).
Prize: X=(?<PX>[0-9]+), Y=(?<PY>[0-9]+)",
    )?;

    let records: Vec<Record> = re
        .captures_iter(&data)
        .map(|cap| {
            let button_a: (u32, u32) = (cap["AX"].parse().unwrap(), cap["AY"].parse().unwrap());
            let button_b: (u32, u32) = (cap["BX"].parse().unwrap(), cap["BY"].parse().unwrap());
            let prize: (u32, u32) = (cap["PX"].parse().unwrap(), cap["PY"].parse().unwrap());
            Record {
                button_a,
                button_b,
                prize,
            }
        })
        .collect();
    // println!("{:?}", cap);
    Ok(records)
}

fn part1(inputfile: &str) -> Result<i32> {
    let records = read_input(inputfile)?;
    records.iter().Ok(0)
}

fn part2(inputfile: &str) -> Result<i32> {
    let input = read_input(inputfile)?;
    Ok(0)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day13_test.txt").unwrap(), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day13_test.txt").unwrap(), 0);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
