use anyhow::Result;
use std::fs;
use std::iter;
use std::iter::zip;

enum Disk {
    File((usize, usize)),
    Empty(usize),
}

fn read_input(inputfile: &str) -> Result<Vec<Disk>> {
    let data = fs::read_to_string(inputfile)?;
    let file_map: Vec<Disk> = data
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(idx, length)| {
            if idx % 2 == 0 {
                Disk::File((idx / 2, length))
            } else {
                Disk::Empty(length)
            }
        })
        .collect();
    Ok(file_map)
}

fn expand_blocks(file_map: &Vec<Disk>) -> Vec<Option<usize>> {
    file_map
        .iter()
        .flat_map(|disk| match disk {
            Disk::File((id, length)) => iter::repeat(Some(*id)).take(*length),
            Disk::Empty(length) => iter::repeat(None).take(*length),
        })
        .collect()
}

fn compress(full_map: &mut Vec<Option<usize>>) {
    let mut first_free = 0;
    let mut last_file = full_map.len() - 1;
    loop {
        while let Some(_) = full_map[first_free] {
            first_free += 1;
        }
        while let None = full_map[last_file] {
            last_file -= 1;
        }
        if first_free >= last_file {
            break;
        }
        full_map.swap(first_free, last_file);
        // print_map(&full_map);
    }
}

fn compress_files(block_map: &mut Vec<Option<usize>>) {
    let mut file_end = block_map.len() - 1;
    let mut moved_files: Vec<usize> = vec![];

    loop {
        // for step in 0..10 {
        while let None = block_map[file_end] {
            if file_end == 0 {
                break;
            }
            file_end -= 1;
        }
        // println!("found file end {file_end}");
        // file_end += 1;
        let mut file_start = file_end;
        while block_map[file_start] == block_map[file_end] {
            if file_start == 0 {
                break;
            }
            file_start -= 1;
        }
        // println!("found file start {file_start}");
        if file_start == 0 {
            break;
        }
        // file_start += 1;
        if moved_files.contains(&block_map[file_start + 1].unwrap()) {
            file_end -= 1;
            continue;
        }
        let file_length = file_end - file_start;
        // println!("start {file_start} end {file_end} length {file_length}");

        let mut space_start = 0;
        let space = loop {
            while let Some(_) = block_map[space_start] {
                space_start += 1;
            }
            // println!("{space_start}");
            if space_start == block_map.len() - 1 {
                break None;
            }
            let mut space_end = space_start;
            while let None = block_map[space_end] {
                if space_end == block_map.len() - 1 {
                    break;
                }
                space_end += 1;
            }
            if space_end - space_start >= file_length {
                break Some((space_start, space_end));
            } else {
                space_start += 1;
            }
        };
        // println!("space = {:?}", space);
        match space {
            Some((start, end)) => {
                if start < file_start + 1 {
                    moved_files.push(block_map[file_start + 1].unwrap());
                    for (src, dest) in zip(start..end, file_start + 1..file_end + 1) {
                        // println!("swap {src} {dest}");
                        block_map.swap(src, dest);
                        // print_map(&block_map);
                    }
                } else {
                    println!(
                        "can't move file {} left",
                        block_map[file_start + 1].unwrap()
                    );
                    file_end = file_start - 1;
                    continue;
                }
            }
            None => {
                println!(
                    "no space big enough for file {}",
                    block_map[file_start + 1].unwrap()
                );
                file_end = file_start - 1;
                continue;
            }
        };
        //     print_map(&block_map);
    }

    // while let Disk::Empty(length) = block_map[last_file] {
    // last_file -= 1;
    // }
    // if let Disk::File((id, length)) = block_map[last_file] {}
}

fn checksum(full_map: &Vec<Option<usize>>) -> usize {
    let mut sum = 0;
    for (pos, file_id) in full_map.iter().enumerate() {
        if file_id.is_some() {
            sum += pos * file_id.unwrap();
        }
    }
    sum
}

fn print_map(full_map: &Vec<Option<usize>>) {
    for entry in full_map {
        match entry {
            Some(id) => print!("{}", id),
            None => print!("."),
        }
    }
    print!("\n");
}

fn part1(inputfile: &str) -> Result<usize> {
    let file_map = read_input(inputfile)?;
    let mut block_map = expand_blocks(&file_map);
    print_map(&block_map);
    compress(&mut block_map);
    print_map(&block_map);
    Ok(checksum(&block_map))
}

fn part2(inputfile: &str) -> Result<usize> {
    let file_map = read_input(inputfile)?;
    let mut block_map = expand_blocks(&file_map);
    print_map(&block_map);
    compress_files(&mut block_map);
    print_map(&block_map);
    Ok(checksum(&block_map))
}

#[test]
fn day9_test_part1() {
    assert_eq!(part1("./input/day9_test.txt").unwrap(), 1928);
}

#[test]
fn day9_test_part2() {
    assert_eq!(part2("./input/day9_test.txt").unwrap(), 2858);
}

pub fn run(inputfile: &str) -> Result<(usize, usize)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
