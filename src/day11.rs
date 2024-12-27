use anyhow::Result;
use std::{collections::HashMap, fs};

fn read_input(inputfile: &str) -> Result<Vec<usize>> {
    let data = fs::read_to_string(inputfile)?;
    let stones: Vec<usize> = data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(stones)
}

fn part1(inputfile: &str, n_blink: usize) -> Result<usize> {
    let mut stones = read_input(inputfile)?;
    for i in 0..n_blink {
        stones = blink(stones);
        // println!("{:?}", stones);
        println!("{} {}", i, stones.len());
    }
    Ok(stones.len())
}

fn part2(inputfile: &str, n_blink: usize) -> Result<usize> {
    let stones = read_input(inputfile)?;
    let mut cache: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        cache.insert(stone, 1);
    }
    for i in 0..n_blink {
        blink2(&mut cache);
        // println!(
        //     "{:?}",
        //     cache.iter().filter(|(&_k, &v)| v != 0).collect::<Vec<_>>()
        // );
        println!("{} {}", i, cache.values().sum::<usize>());
    }
    Ok(cache.values().sum::<usize>())
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    stones
        .into_iter()
        .flat_map(|s| {
            let s_str = s.to_string();
            if s == 0 {
                vec![1]
            } else if s_str.len() % 2 == 0 {
                // println!(
                //     "s_str={} len={} l={} r={}",
                //     s_str,
                //     s_str.len(),
                //     s_str[..s_str.len() / 2].to_string(),
                //     s_str[s_str.len() / 2..].to_string()
                // );
                let l: usize = s_str[..s_str.len() / 2].parse().unwrap();
                let r: usize = s_str[s_str.len() / 2..].parse().unwrap();
                vec![l, r]
            } else {
                vec![s * 2024]
            }
        })
        .collect()
}

fn blink2(stones: &mut HashMap<usize, usize>) {
    for (&s, &count) in stones.clone().iter() {
        if count == 0 {
            continue;
        }
        let s_str = s.to_string();
        if s == 0 {
            stones.entry(1).and_modify(|e| *e += count).or_insert(count);
        } else if s_str.len() % 2 == 0 {
            let l: usize = s_str[..s_str.len() / 2].parse().unwrap();
            let r: usize = s_str[s_str.len() / 2..].parse().unwrap();
            // println!(
            //     "s={} l={} r={} count={} half_count={}",
            //     s, l, r, count, half_count
            // );
            stones.entry(l).and_modify(|e| *e += count).or_insert(count);
            stones.entry(r).and_modify(|e| *e += count).or_insert(count);
        } else {
            stones
                .entry(s * 2024)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
        // println!("stones={:?}, s={}, count={}", stones, s, count);
        stones.entry(s).and_modify(|e| *e -= count);
    }
}

#[test]
fn test_blink_once() {
    let stones: Vec<usize> = vec![0, 1, 10, 99, 999];
    let new_stones = blink(stones);
    assert_eq!(new_stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
}

#[test]
fn test_blink_multiple() {
    let mut stones: Vec<usize> = vec![125, 17];

    for _ in 0..6 {
        stones = blink(stones);
    }
    assert_eq!(
        stones,
        vec![
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2
        ]
    );
    assert_eq!(stones.len(), 22);
}

#[test]
fn test_blink2() {
    let stones: Vec<usize> = vec![0, 1, 10, 99, 999];
    let mut cache: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        cache.insert(stone, 1);
    }
    blink2(&mut cache);
    println!("{:?}", cache);
    println!("{}", cache.values().sum::<usize>());
    assert_eq!(cache.values().sum::<usize>(), 7);
}

#[test]
fn test_blink2_multiple() {
    let stones: Vec<usize> = vec![125, 17];
    let mut cache: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        cache.insert(stone, 1);
    }
    for _ in 0..6 {
        blink2(&mut cache);
        println!(
            "{:?}",
            cache.iter().filter(|(&_k, &v)| v != 0).collect::<Vec<_>>()
        );
    }
    println!("{:?}", cache);
    println!("{}", cache.values().sum::<usize>());
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day11_test.txt", 25).unwrap(), 55312);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day11_test.txt", 25).unwrap(), 55312);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile, 25)? as i32, part2(inputfile, 75)? as i32))
}
