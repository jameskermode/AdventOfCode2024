use anyhow::Result;
use itertools::{repeat_n, Itertools};
use std::fs;
use std::iter::zip;

fn read_input(inputfile: &str) -> Result<(Vec<i64>, Vec<Vec<i64>>)> {
    let data = fs::read_to_string(inputfile)?;
    let mut totals: Vec<i64> = vec![];
    let mut all_terms: Vec<Vec<i64>> = vec![];
    for line in data.lines() {
        let mut fields = line.split_whitespace();
        let mut total = fields.next().unwrap().to_string();
        total.pop(); // remove the trailing colon
        totals.push(total.parse().unwrap());
        let mut v: Vec<i64> = vec![];
        for term in fields {
            v.push(term.parse().unwrap());
        }
        all_terms.push(v);
    }
    Ok((totals, all_terms))
}

fn check(total: i64, terms: &Vec<i64>, ops: &[char]) -> bool {
    let n_op = terms.len() - 1;
    let perms: Vec<_> = repeat_n(ops, n_op).multi_cartesian_product().collect();
    for perm in perms {
        let mut nums = terms.clone().into_iter();
        let mut result = nums.next().unwrap();
        for op in perm {
            result = match op {
                '+' => result + nums.next().unwrap(),
                '*' => result * nums.next().unwrap(),
                '|' => (result.to_string() + &nums.next().unwrap().to_string())
                    .parse::<i64>()
                    .unwrap(),
                _ => panic!("bad operator"),
            }
        }
        if result == total {
            return true;
        }
        // println!("{}", result)
    }
    false
}

fn part1(inputfile: &str) -> Result<i64> {
    let (totals, all_terms) = read_input(inputfile)?;
    let mut sum_valid = 0;
    for (total, terms) in zip(totals, all_terms) {
        let valid = check(total, &terms, &['+', '*']);
        if valid {
            sum_valid += total
        }
        println!("{} {:?} {}", total, terms, valid);
    }
    Ok(sum_valid)
}

fn part2(inputfile: &str) -> Result<i64> {
    let (totals, all_terms) = read_input(inputfile)?;
    let mut sum_valid = 0;
    for (total, terms) in zip(totals, all_terms) {
        let valid = check(total, &terms, &['+', '*', '|']);
        if valid {
            sum_valid += total
        }
        println!("{} {:?} {}", total, terms, valid);
    }
    Ok(sum_valid)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day7_test.txt").unwrap(), 3749);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day7_test.txt").unwrap(), 11387);
}

pub fn run(inputfile: &str) -> Result<(i64, i64)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
