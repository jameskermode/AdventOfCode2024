use anyhow::Result;
use grid::Grid;
use itertools::Itertools;
use std::fs;

fn read_input(inputfile: &str) -> Result<Grid<char>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data.chars().filter(|&c| c != '\n').collect();
    let grid = Grid::from_vec(chars, data.lines().next().unwrap().len());
    Ok(grid)
}

fn area(grid: &Grid<char>, garden: char) -> usize {
    grid.iter().filter(|&&ch| ch == garden).count()
}

fn out_neighbours(grid: &Grid<char>, row: usize, col: usize) -> usize {
    let val = grid[(row, col)];
    let row: i32 = row as i32;
    let col: i32 = col as i32;
    let n: Vec<(i32, i32)> = vec![
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ];
    n.iter()
        .map(|&idx| {
            println!(
                "{:?} {:?} {}",
                idx,
                grid.get(idx.0, idx.1),
                *grid.get(idx.0, idx.1).unwrap_or(&'.') != val
            );
            *(grid.get(idx.0, idx.1).unwrap_or(&'.')) != val
        })
        .count()
}

fn perimeter(grid: &Grid<char>, garden: char) -> usize {
    return grid
        .indexed_iter()
        .filter_map(|((r, c), &val)| {
            if val == garden {
                println!("{} {} {} {}", val, r, c, out_neighbours(&grid, r, c));
                Some(out_neighbours(&grid, r, c))
            } else {
                None
            }
        })
        .sum();
}

fn part1(inputfile: &str) -> Result<usize> {
    let grid = read_input(inputfile)?;
    let gardens: Vec<_> = grid.iter().unique().collect();
    let price = gardens.iter().fold(0, |price, &&ch| {
        let a = area(&grid, ch);
        let p = perimeter(&grid, ch);
        println!("garden {} area {} perimeter {}", ch, a, p);
        price + a * p
    });
    println!("{:?}", gardens);
    Ok(price)
}

fn part2(inputfile: &str) -> Result<i32> {
    // let ... = read_input(inputfile)?;
    // Ok(...)
    todo!()
}

#[test]
fn test_part1_0() {
    assert_eq!(part1("./input/day12_test_0.txt").unwrap(), 772);
}

#[test]
fn test_part1_1() {
    assert_eq!(part1("./input/day12_test_1.txt").unwrap(), 772);
}

#[test]
fn test_part1_2() {
    assert_eq!(part1("./input/day12_test_2.txt").unwrap(), 1930);
}

// #[test]
// fn test_part2() {
// assert_eq!(part2("./input/day12_test.txt").unwrap(), 0);
// }

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)? as i32, part2(inputfile)?))
}
