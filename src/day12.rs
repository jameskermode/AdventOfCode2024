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

fn flood(grid: &mut Grid<char>, origin: (usize, usize), value: char, fill: char) {
    match grid.get(origin.0, origin.1) {
        Some(&origin_value) => {
            if origin_value != value {
                return;
            }
        }
        None => return,
    }
    grid[(origin.0, origin.1)] = fill;
    let origin: (isize, isize) = (origin.0 as isize, origin.1 as isize);

    for next_origin in [
        (origin.0 - 1, origin.1),
        (origin.0 + 1, origin.1),
        (origin.0, origin.1 - 1),
        (origin.0, origin.1 + 1),
    ] {
        flood(
            grid,
            (next_origin.0 as usize, next_origin.1 as usize),
            value,
            fill,
        );
    }
}

fn find_regions(grid: &Grid<char>, garden: char) -> Vec<Grid<char>> {
    let mut regions: Vec<_> = vec![];
    let mut visited: Grid<Option<bool>> = Grid::new(grid.rows(), grid.cols());
    visited.fill(None);
    for ((r, c), &val) in grid.indexed_iter() {
        if val == garden {
            visited[(r, c)] = Some(false);
        }
    }

    loop {
        let Some(origin) = visited
            .indexed_iter()
            .filter_map(|(idx, &e)| if e == Some(false) { Some(idx) } else { None })
            .next()
        else {
            break;
        };
        let mut region = grid.clone();
        flood(&mut region, origin, garden, '.');
        for ((r, c), &ch) in region.indexed_iter() {
            if ch == '.' {
                visited[(r, c)] = Some(true);
            }
        }
        regions.push(region);
    }
    regions
}

fn area(region: &Grid<char>, marker: char) -> usize {
    region.iter().filter(|&&val| val == marker).count()
}

fn neighbours(row: usize, col: usize) -> [(usize, usize); 4] {
    let row: isize = row as isize;
    let col: isize = col as isize;
    [
        ((row - 1) as usize, col as usize), // North
        (row as usize, (col - 1) as usize), // West
        ((row + 1) as usize, col as usize), // South
        (row as usize, (col + 1) as usize), // East
    ]
}

fn perimeter(grid: &Grid<char>, marker: char) -> usize {
    grid.indexed_iter()
        .filter_map(|((r, c), &val)| {
            if val == marker {
                // println!("{} {} {} {}", val, r, c, out_neighbours(&grid, r, c));
                let n_out = neighbours(r, c)
                    .iter()
                    .filter(|(r, c)| *grid.get(*r, *c).unwrap_or(&'#') != marker)
                    .count();
                Some(n_out)
            } else {
                None
            }
        })
        .sum()
}

fn corners(grid: &Grid<char>, marker: char) -> usize {
    let mut corners = 0;

    for ((r, c), &val) in grid.indexed_iter() {
        if val == marker {
            corners += neighbours(r, c)
                .iter()
                .circular_tuple_windows()
                .filter_map(|(&n1, &n2)| {
                    // println!("rc {:?} n1 {:?} n2 {:?}", (r, c), n1, n2);
                    let n1_out = *grid.get(n1.0, n1.1).unwrap_or(&'#') != marker;
                    let n2_out = *grid.get(n2.0, n2.1).unwrap_or(&'#') != marker;
                    if n1_out && n2_out {
                        println!("exterior corner at {:?}", (r, c));
                        Some(1)
                    } else if !n1_out && !n2_out {
                        let n3 = (n1.0 + n2.0 - r, n1.1 + n2.1 - c);
                        // println!(
                        //     "checking rc={:?} at n1={:?}, n2={:?}, n3={:?}",
                        //     (r, c),
                        //     n1,
                        //     n2,
                        //     n3
                        // );
                        if let Some(&n3_val) = grid.get(n3.0, n3.1) {
                            if n3_val != marker {
                                println!("interior corner at {:?}", n3);
                                Some(1)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .sum::<usize>();
        }
    }
    corners
}

fn part1(inputfile: &str) -> Result<usize> {
    let grid = read_input(inputfile)?;
    let gardens: Vec<_> = grid.iter().unique().collect();
    let mut price = 0;
    for &garden in gardens {
        for (r, region) in find_regions(&grid, garden).iter().enumerate() {
            let a = area(&region, '.');
            let p = perimeter(&region, '.');
            println!("garden {} region {} area {} perimeter {}", garden, r, a, p);
            price += a * p;
        }
    }
    Ok(price)
}

fn part2(inputfile: &str) -> Result<usize> {
    let grid = read_input(inputfile)?;
    let gardens: Vec<_> = grid.iter().unique().collect();
    let mut price = 0;
    for &garden in gardens {
        for (r, region) in find_regions(&grid, garden).iter().enumerate() {
            let a = area(&region, '.');
            let c = corners(&region, '.');
            println!("garden {} region {} area {} corners {}", garden, r, a, c);
            price += a * c;
        }
    }
    Ok(price)
}

#[test]
fn test_part1_0() {
    assert_eq!(part1("./input/day12_test_0.txt").unwrap(), 140);
}

#[test]
fn test_part1_1() {
    assert_eq!(part1("./input/day12_test_1.txt").unwrap(), 772);
}

#[test]
fn test_part1_2() {
    assert_eq!(part1("./input/day12_test_2.txt").unwrap(), 1930);
}

#[test]
fn test_part2_0() {
    assert_eq!(part2("./input/day12_test_0.txt").unwrap(), 80);
}

#[test]
fn test_part2_1() {
    assert_eq!(part2("./input/day12_test_1.txt").unwrap(), 436);
}

#[test]
fn test_part2_2() {
    assert_eq!(part2("./input/day12_test_2.txt").unwrap(), 1206);
}

// #[test]
// fn test_part2_3() {
// assert_eq!(part2("./input/day12_test_3.txt").unwrap(), 236);
// }

// #[test]
// fn test_part2_4() {
// assert_eq!(part2("./input/day12_test_4.txt").unwrap(), 368);
// }

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)? as i32, part2(inputfile)? as i32))
}
