use anyhow::Result;
use std::fs;

fn read_data(inputfile: &str) -> Result<Vec<Vec<i32>>> {
    let data = fs::read_to_string(inputfile)?;
    let mut vecs = Vec::<Vec<i32>>::new();
    for line in data.lines() {
        let mut vec = Vec::<i32>::new();
        for field in line.split_whitespace() {
            vec.push(field.parse()?);
        }
        vecs.push(vec)
    }
    Ok(vecs)
}

fn is_safe(vec: &Vec<i32>) -> bool {
    let diff: Vec<i32> = vec.windows(2).map(|s| s[1] - s[0]).collect();
    let monotonic = diff.iter().all(|x| *x > 0) || diff.iter().all(|x| *x < 0);
    let max_abs_diff = diff.iter().map(|x| x.abs()).max().unwrap();
    monotonic && max_abs_diff >= 1 && max_abs_diff <= 3
}

fn part1(inputfile: &str) -> Result<i32> {
    let vecs = read_data(inputfile)?;
    // println!("{:?}", vecs);
    let n_safe: i32 = vecs.into_iter().filter(|vec| is_safe(vec)).count() as i32;
    Ok(n_safe)
}

fn part2(inputfile: &str) -> Result<i32> {
    let vecs = read_data(inputfile)?;
    // println!("{:?}", vecs);
    let mut n_safe: i32 = 0;
    for vec in vecs {
        for skip_index in 0..vec.len() {
            let mut newvec = vec.clone();
            newvec.remove(skip_index);
            // let safe = is_safe(&newvec);
            // println!("{:?} {safe}", newvec);
            if is_safe(&newvec) {
                n_safe += 1;
                break;
            }
        }
    }
    Ok(n_safe)
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day2_test.txt").unwrap(), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day2_test.txt").unwrap(), 4);
}
