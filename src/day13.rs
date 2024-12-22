use anyhow::Result;
use ndarray::prelude::*;
use ndarray_linalg::solve::Solve;
use regex::Regex;
use std::fs;

#[derive(Debug)]
#[allow(non_snake_case)]
struct Record {
    A: Array2<f64>,
    prize: Array1<f64>,
}

fn read_input(inputfile: &str) -> Result<Vec<Record>> {
    let data = fs::read_to_string(inputfile)?;

    let re = Regex::new(
        r"(?ms)Button A: X\+(?<AX>[0-9]+), Y\+(?<AY>[0-9]+)
Button B: X\+(?<BX>[0-9]+), Y\+(?<BY>[0-9]+)
Prize: X=(?<PX>[0-9]+), Y=(?<PY>[0-9]+)",
    )?;

    let records: Vec<Record> = re
        .captures_iter(&data)
        .map(|cap| {
            // println!("{:?}", cap);
            #[allow(non_snake_case)]
            let A: Array2<f64> = array![
                [cap["AX"].parse().unwrap(), cap["BX"].parse().unwrap()],
                [cap["AY"].parse().unwrap(), cap["BY"].parse().unwrap()]
            ];
            let prize: Array1<f64> = array![
                cap["PX"].parse::<f64>().unwrap(),
                cap["PY"].parse::<f64>().unwrap()
            ];
            Record { A, prize }
        })
        .collect();
    // println!("{:?}", cap);
    Ok(records)
}

fn solve(records: &Vec<Record>, shift: &Array1<f64>) -> Result<i64> {
    let cost: Array1<i64> = array![3, 1];
    let mut total = 0;
    for record in records {
        // println!("{:?}", record.A);
        let b = record.prize.clone() + shift;
        let n: Array1<f64> = record.A.solve(&b)?;
        let max_abs_diff =
            (n.mapv(|x| x.round()) - n.clone()).fold(f64::NEG_INFINITY, |m, &v| m.max(v.abs()));

        // println!("n = {:?}, diff = {:?}", n, max_abs_diff);

        if max_abs_diff < 1e-4 {
            let n: Array1<i64> = n.mapv(|x| x.round() as i64);
            total += n.dot(&cost);
        }
    }
    Ok(total)
}

fn part1(inputfile: &str) -> Result<i64> {
    let records = read_input(inputfile)?;
    let total = solve(&records, &array![0.0, 0.0]);
    total
}

fn part2(inputfile: &str) -> Result<i64> {
    let records = read_input(inputfile)?;
    let total = solve(&records, &array![10000000000000.0, 10000000000000.0]);
    total
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day13_test.txt").unwrap(), 480);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day13_test.txt").unwrap(), 875318608908);
}

pub fn run(inputfile: &str) -> Result<(i64, i64)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
