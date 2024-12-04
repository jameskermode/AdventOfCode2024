use anyhow::Result;
use std::fs;

fn _print_grid(lines: &Vec<&[u8]>) {
    for line in lines {
        for idx in 1..line.len() {
            let ch = line[idx] as char;
            print!("{}", ch);
        }
        print!("\n");
    }
}

fn count_words(lines: &Vec<&[u8]>, pattern: &[u8], i: usize, j: usize) -> usize {
    let w = lines[0].len();
    let h = lines.len();
    let mut count = 0;
    for (dx, dy) in [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ] {
        let mut matched = true;
        for (k, pat_letter) in pattern.into_iter().enumerate() {
            let k = k as i32;
            let x = i as i32 + k * dx;
            let y = j as i32 + k * dy;
            if (x < 0)
                || (x as usize >= w)
                || (y < 0)
                || (y as usize >= h)
                || (lines[x as usize][y as usize] != *pat_letter)
            {
                matched = false;
                break;
            }
        }
        if matched {
            count += 1;
        }
    }
    count
}

fn part1(inputfile: &str) -> Result<i32> {
    let data = fs::read_to_string(inputfile)?;
    let lines: Vec<&[u8]> = data.lines().map(|s| s.as_bytes()).collect();
    let mut total: usize = 0;
    // _print_grid(&lines);
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] as char == 'X' {
                let c = count_words(&lines, &("XMAS".as_bytes()), i, j);
                total += c;
            }
        }
    }
    Ok(total as i32)
}

#[rustfmt::skip]
fn check_mas(lines: &Vec<&[u8]>, i: usize, j: usize) -> bool {
    let a = ((lines[i - 1][j - 1] == 'M' as u8) && (lines[i + 1][j + 1] == 'S' as u8))
         || ((lines[i - 1][j - 1] == 'S' as u8) && (lines[i + 1][j + 1] == 'M' as u8));
    let b = ((lines[i + 1][j - 1] == 'M' as u8) && (lines[i - 1][j + 1] == 'S' as u8))
         || ((lines[i + 1][j - 1] == 'S' as u8) && (lines[i - 1][j + 1] == 'M' as u8));
    a && b
}

fn part2(inputfile: &str) -> Result<i32> {
    let data = fs::read_to_string(inputfile)?;
    let lines: Vec<&[u8]> = data.lines().map(|s| s.as_bytes()).collect();
    let mut total: usize = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            if lines[i][j] as char == 'A' && check_mas(&lines, i, j) {
                total += 1;
            }
        }
    }
    Ok(total as i32)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day4_test.txt").unwrap(), 18);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day4_test.txt").unwrap(), 9);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
