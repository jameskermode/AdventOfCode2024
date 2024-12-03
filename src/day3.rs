use anyhow::Result;
use regex::Regex;
use std::fs;

fn part1(inputfile: &str) -> Result<i32> {
    let data = fs::read_to_string(inputfile)?;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let result: i32 = re
        .captures_iter(&data)
        .map(|caps| {
            let (m, [a, b]) = caps.extract();
            println!("{m} {a} {b}");
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();
            a * b
        })
        .sum();
    Ok(result)
}

#[derive(Debug, Clone)]
enum State {
    DO,
    DONT,
    MUL,
}

#[derive(Debug, Clone)]
struct Record {
    start: usize,
    state: State,
    value: i32,
}

fn part2(inputfile: &str) -> Result<i32> {
    let data = fs::read_to_string(inputfile)?;
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let do_re = Regex::new(r"do\(\)")?;
    let dont_re = Regex::new(r"don't\(\)")?;
    let mut do_vec: Vec<Record> = do_re
        .find_iter(&data)
        .map(|m| Record {
            start: m.start(),
            state: State::DO,
            value: 0,
        })
        .collect();
    let mut dont_vec: Vec<Record> = dont_re
        .find_iter(&data)
        .map(|m| Record {
            start: m.start(),
            state: State::DONT,
            value: 0,
        })
        .collect();
    let mut mul_vec: Vec<Record> = mul_re
        .captures_iter(&data)
        .map(|c| {
            let m = c.get(0).unwrap();
            let (_, [a, b]) = c.extract();
            Record {
                start: m.start(),
                state: State::MUL,
                value: a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap(),
            }
        })
        .collect();

    let mut vec: Vec<Record> = vec![];
    vec.append(&mut mul_vec);
    vec.append(&mut do_vec);
    vec.append(&mut dont_vec);
    vec.sort_by(|a, b| a.start.cmp(&b.start));
    // println!("{:?}", vec);

    let mut state = State::DO;
    let mut total: i32 = 0;
    for record in vec {
        // println!("{state:?} {record:?}");
        state = match record.state {
            State::DO => record.state,
            State::DONT => record.state,
            State::MUL => match state {
                State::DO => {
                    total += record.value;
                    state
                }
                _ => state,
            },
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
