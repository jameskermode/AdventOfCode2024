use anyhow::Result;
use grid::Grid;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::fs;
// use std::io;
// use std::io::prelude::*;

fn read_grid(inputfile: &str) -> Result<Grid<char>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data.chars().filter(|c| *c != '\n').collect();
    let cols = data.lines().next().unwrap().len();
    Ok(Grid::from_vec(chars, cols))
}

fn _print_grid(grid: &Grid<char>) {
    // print!("{}[2J", 27 as char);
    for rowidx in 0..grid.rows() {
        let rowstr: String = grid.iter_row(rowidx).collect();
        println!("{rowstr}");
    }
}

fn char_to_dir(ch: char) -> (isize, isize) {
    match ch {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => (0, 0),
    }
}

fn step(grid: &mut Grid<char>, guard: Option<(usize, usize)>) -> Option<(usize, usize)> {
    let directions = vec!['^', '>', 'v', '<'];
    let guard = guard.unwrap();
    let mut guard_ch = *grid.get(guard.0, guard.1).unwrap();
    let mut dir = char_to_dir(guard_ch);

    let (new_guard, new_guard_ch) =
        match grid.get(guard.0 as isize + dir.0, guard.1 as isize + dir.1) {
            Some('.') | Some('X') => (
                Some((
                    (guard.0 as isize + dir.0) as usize,
                    (guard.1 as isize + dir.1) as usize,
                )),
                guard_ch,
            ),
            Some('#') => {
                while *grid
                    .get(guard.0 as isize + dir.0, guard.1 as isize + dir.1)
                    .unwrap()
                    == '#'
                {
                    guard_ch = directions
                        [(directions.iter().position(|&ch| ch == guard_ch).unwrap() + 1) % 4];
                    dir = char_to_dir(guard_ch);
                }
                (
                    Some((
                        (guard.0 as isize + dir.0) as usize,
                        (guard.1 as isize + dir.1) as usize,
                    )),
                    guard_ch,
                )
            }
            _ => (None, guard_ch),
        };

    grid[(guard.0, guard.1)] = 'X';
    if new_guard.is_some() {
        assert!(grid[(new_guard.unwrap().0, new_guard.unwrap().1)] != '#');
        grid[(new_guard.unwrap().0, new_guard.unwrap().1)] = new_guard_ch;
    }
    new_guard
}

fn part1(inputfile: &str) -> Result<usize> {
    let mut grid = read_grid(inputfile)?;
    let mut guard = None;
    for ((row, col), &val) in grid.indexed_iter() {
        if val == '^' {
            guard = Some((row, col));
            break;
        }
    }
    // println!("Guard is at {guard:?}");
    while guard.is_some() {
        guard = step(&mut grid, guard);
        // print_grid(&grid);
    }
    let npos = grid.iter().filter(|&c| *c == 'X').count();
    println!("{npos}");
    Ok(npos)
}

fn part2(inputfile: &str) -> Result<i32> {
    let orig_grid = read_grid(inputfile)?;
    let mut orig_guard = None;
    for ((row, col), &val) in orig_grid.indexed_iter() {
        if val == '^' {
            orig_guard = Some((row, col));
            break;
        }
    }
    let mut ncycles = 0;
    let bar = ProgressBar::new((orig_grid.rows() * orig_grid.cols()) as u64);
    for ((row, col), &val) in orig_grid.indexed_iter() {
        bar.inc(1);
        if val != '.' {
            continue;
        }

        let mut grid = orig_grid.clone();
        let mut dir_visited = HashMap::<char, Grid<bool>>::new();
        for dir in ['^', '>', 'v', '<'] {
            let mut g = Grid::new(orig_grid.rows(), orig_grid.cols());
            g.fill(false);
            dir_visited.insert(dir, g);
        }
        let mut guard = orig_guard;
        grid[(row, col)] = '#';

        let mut cycle = false;
        while guard.is_some() && !cycle {
            guard = step(&mut grid, guard);
            if guard.is_none() {
                break;
            }
            let pos = guard.unwrap();
            let dir = grid[(pos.0, pos.1)];
            if dir_visited[&dir][(pos.0, pos.1)] {
                cycle = true;
                break;
            }
            let dict = dir_visited.get_mut(&dir).unwrap();
            dict[pos] = true;
        }
        if cycle {
            ncycles += 1
        }
    }
    bar.finish();
    println!("{ncycles}");
    Ok(ncycles)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day6_test.txt").unwrap(), 41);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day6_test.txt").unwrap(), 6);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)? as i32, part2(inputfile)?))
}
