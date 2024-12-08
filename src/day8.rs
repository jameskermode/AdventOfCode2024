use anyhow::Result;
use grid::Grid;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

fn read_input(inputfile: &str) -> Result<Grid<char>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data.chars().filter(|c| *c != '\n').collect();
    let cols = data.lines().next().unwrap().len();
    Ok(Grid::from_vec(chars, cols))
}

fn print_grid(grid: &Grid<char>) {
    // print!("{}[2J", 27 as char);
    for rowidx in 0..grid.rows() {
        let rowstr: String = grid.iter_row(rowidx).collect();
        println!("{rowstr}");
    }
}

fn find_stations(grid: &Grid<char>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut stations = HashMap::<char, Vec<(i32, i32)>>::new();
    for ((i, j), &val) in grid.indexed_iter() {
        match val {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                match stations.get_mut(&val) {
                    Some(v) => v.push((i as i32, j as i32)),
                    None => {
                        stations.insert(val, vec![(i as i32, j as i32)]);
                    }
                };
            }
            _ => continue,
        }
    }
    stations
}

fn find_antinodes(
    grid: &mut Grid<char>,
    stations: &HashMap<char, Vec<(i32, i32)>>,
    part2: bool,
) -> Vec<(i32, i32)> {
    let mut anti: Vec<(i32, i32)> = vec![];
    for (station, coords) in stations.iter() {
        println!("{:?}", station);
        for i1 in 0..coords.len() {
            for i2 in 0..coords.len() {
                if i1 == i2 {
                    continue;
                }
                let d = (coords[i1].0 - coords[i2].0, coords[i1].1 - coords[i2].1);

                let mut nrange = 1..=1;
                if part2 {
                    nrange = 0..=100; // over-estimate
                }
                for n in nrange {
                    for (i, s) in zip([i1, i2], [1, -1]) {
                        let a = (coords[i].0 + n * s * d.0, coords[i].1 + n * s * d.1);
                        let r = grid.get_mut(a.0 as usize, a.1 as usize);
                        if r.is_some() {
                            *(r.unwrap()) = '#';
                            if !anti.contains(&a) {
                                anti.push(a);
                            }
                            // nanti = nanti + 1
                        }
                    }
                }

                // println!("{} {:?} {} {:?} d {:?}", i1, coords[i1], i2, coords[i2], d);
            }
        }
    }
    anti
}

fn part1(inputfile: &str) -> Result<i32> {
    let mut grid = read_input(inputfile)?;
    // print_grid(&grid);
    let stations = find_stations(&grid);
    // println!("{:?}", stations);
    print_grid(&grid);
    let anti = find_antinodes(&mut grid, &stations, false);
    print_grid(&grid);
    println!("{}", grid.iter().filter(|&c| *c == '#').count());
    Ok(anti.len() as i32)
}

fn part2(inputfile: &str) -> Result<i32> {
    let mut grid = read_input(inputfile)?;
    // print_grid(&grid);
    let stations = find_stations(&grid);
    // println!("{:?}", stations);
    print_grid(&grid);
    let anti = find_antinodes(&mut grid, &stations, true);
    print_grid(&grid);
    println!("{}", grid.iter().filter(|&c| *c == '#').count());
    Ok(anti.len() as i32)
}

#[test]
fn test_part1_v1() {
    assert_eq!(part1("./input/day8_test1.txt").unwrap(), 2);
}

#[test]
fn test_part1_v2() {
    assert_eq!(part1("./input/day8_test2.txt").unwrap(), 4);
}

#[test]
fn test_part1_v3() {
    assert_eq!(part1("./input/day8_test3.txt").unwrap(), 4);
}

#[test]
fn test_part1_full() {
    assert_eq!(part1("./input/day8_test.txt").unwrap(), 14);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day8_test.txt").unwrap(), 34);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
