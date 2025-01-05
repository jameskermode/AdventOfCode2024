use anyhow::Result;
use std::fs;
use std::iter;

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

fn compress_files(file_map: &Vec<Disk>, block_map: &mut Vec<Option<usize>>) {
    for file in file_map.iter().rev() {
        if let Disk::File((id, length)) = file {
            let length = *length;
            let file_start = block_map
                .iter()
                .position(|block| {
                    if let Some(file_id) = block {
                        file_id == id
                    } else {
                        false
                    }
                })
                .unwrap();

            for window in block_map[..file_start]
                .iter()
                .enumerate()
                .collect::<Vec<_>>()
                .windows(length)
            {
                if window.iter().all(|(_idx, block)| block.is_none()) {
                    let space_start = window[0].0;
                    // println!("moving {:?}", &block_map[file_start..file_start + length]);
                    for i in 0..length {
                        block_map.swap(space_start + i, file_start + i);
                    }
                    // alternative: doesn't seem to be faster!
                    // let (left, right) = block_map.split_at_mut(space_start + length);
                    // left[space_start..space_start + length].swap_with_slice(
                    // &mut right[file_start - space_start - length..file_start - space_start],
                    // );
                    break;
                }
            }
            // print_map(block_map);
        }
    }
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

fn _print_map(full_map: &Vec<Option<usize>>) {
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
    // _print_map(&block_map);
    compress(&mut block_map);
    // _print_map(&block_map);
    Ok(checksum(&block_map))
}

fn part2(inputfile: &str) -> Result<usize> {
    let file_map = read_input(inputfile)?;
    let mut block_map = expand_blocks(&file_map);
    // _print_map(&block_map);
    compress_files(&file_map, &mut block_map);
    // _print_map(&block_map);
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
