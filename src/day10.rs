use anyhow::Result;
use grid::Grid;
use std::fs;

fn read_input(inputfile: &str) -> Result<Grid<u32>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let grid = Grid::from_vec(chars, data.lines().next().unwrap().len());
    Ok(grid)
}

fn _print_grid(grid: &Grid<u32>) {
    // print!("{}[2J", 27 as char);
    for rowidx in 0..grid.rows() {
        let rowstr: String = grid.iter_row(rowidx).map(|c| c.to_string()).collect();
        println!("{rowstr}");
    }
}

fn neighbours(grid: Grid<i32>, row: usize, col: usize) -> Vec<Option<i32>> {
    // let val = grid[(row, col)];
    // vec![
    // (row - 1, col),
    // (row + 1, col),
    // (row, col - 1),
    // (row, col + 1),
    // ]
    // .iter()
    // .flat_map(|(r, c)| {
    // if let Some(v) = grid.get(r, c) {
    // if (v - val).abs() == 1 {
    // Some((r, c))
    // } else {
    // None
    // }
    // }
    // })
    todo!()
}

fn map_trail(grid: &Grid<u32>, trail_head: (usize, usize)) {}

fn part1(inputfile: &str) -> Result<i32> {
    let grid = read_input(inputfile)?;
    _print_grid(&grid);
    let trail_heads: Vec<_> = grid
        .indexed_iter()
        .filter_map(|((r, c), &v)| if v == 0 { Some((r, c)) } else { None })
        .collect();
    println!("{:?}", trail_heads);
    for trail_head in trail_heads {}
    Ok(0)
}

fn part2(inputfile: &str) -> Result<i32> {
    let grid = read_input(inputfile)?;
    Ok(0)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day10_test.txt").unwrap(), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day10_test.txt").unwrap(), 0);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
