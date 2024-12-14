use anyhow::Result;
use grid::Grid;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn read_input(inputfile: &str) -> Result<Vec<Robot>> {
    let data = fs::read_to_string(inputfile)?;
    let robots: Vec<Robot> = data
        .lines()
        .map(|line| {
            let mut pos: Option<(i32, i32)> = None;
            let mut vel: Option<(i32, i32)> = None;
            for field in line.split_ascii_whitespace() {
                let (name, value) = field.split_once('=').unwrap();
                let (x, y) = value.split_once(',').unwrap();
                // println!("x = {}, y= {}", x, y);
                let x: i32 = x.parse().unwrap();
                let y: i32 = y.parse().unwrap();
                match name {
                    "p" => pos = Some((x, y)),
                    "v" => vel = Some((x, y)),
                    _ => (),
                }
            }
            Robot {
                pos: pos.unwrap(),
                vel: vel.unwrap(),
            }
        })
        .collect();
    Ok(robots)
}

fn step(robot: &Robot, grid_size: (usize, usize)) -> Robot {
    let mut pos = (robot.pos.0 + robot.vel.0, robot.pos.1 + robot.vel.1);
    if pos.0 < 0 {
        pos.0 += grid_size.0 as i32;
    }
    if pos.0 >= grid_size.0 as i32 {
        pos.0 -= grid_size.0 as i32;
    }
    if pos.1 < 0 {
        pos.1 += grid_size.1 as i32;
    }
    if pos.1 >= grid_size.1 as i32 {
        pos.1 -= grid_size.1 as i32;
    }
    Robot {
        pos,
        vel: robot.vel,
    }
}

fn calc_grid(robots: &Vec<Robot>, grid_size: (usize, usize)) -> Grid<usize> {
    let mut grid = Grid::<usize>::new(grid_size.0, grid_size.1);
    for r in robots {
        grid[(r.pos.0 as usize, r.pos.1 as usize)] += 1;
    }
    grid
}

#[derive(PartialEq, Eq, Hash)]
enum Quad {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

fn quad(r: usize, c: usize, h: usize, w: usize) -> Quad {
    if r < h / 2 && c < w / 2 {
        Quad::TopLeft
    } else if r < h / 2 && c > w / 2 {
        Quad::TopRight
    } else if r > h / 2 && c < w / 2 {
        Quad::BottomLeft
    } else if r > h / 2 && c > w / 2 {
        Quad::BottomRight
    } else {
        Quad::None
    }
}

fn score(grid: &Grid<usize>) -> usize {
    let mut count = HashMap::<Quad, usize>::new();
    for ((r, c), &val) in grid.indexed_iter() {
        let q = quad(r, c, grid.rows(), grid.cols());
        count.entry(q).and_modify(|c| *c += val).or_insert(val);
    }
    count
        .iter()
        .filter_map(|(k, v)| if *k == Quad::None { None } else { Some(v) })
        .product()
}

fn print_grid(grid: &Grid<usize>) {
    // print!("{}[2J", 27 as char);
    for colidx in 0..grid.cols() {
        let colstr: String = grid
            .iter_col(colidx)
            .map(|&v| match v {
                0 => '.',
                _ => char::from_digit(v as u32, 10).unwrap(),
            })
            .collect::<String>();
        println!("{colstr}");
    }
}

fn part1(inputfile: &str, grid_size: (usize, usize)) -> Result<i32> {
    let mut robots = read_input(inputfile)?;
    println!("robots {:?}", robots);
    let grid = calc_grid(&robots, grid_size);
    println!("Intial grid:");
    print_grid(&grid);
    for _idx in 0..100 {
        robots = robots.iter().map(|r| step(r, grid_size)).collect();
    }
    println!("\nFinal grid:");
    let grid = calc_grid(&robots, grid_size);
    print_grid(&grid);
    Ok(score(&grid) as i32)
}

fn part2(inputfile: &str, grid_size: (usize, usize)) -> Result<i32> {
    let mut robots = read_input(inputfile)?;
    println!("robots {:?}", robots);
    let mut grid = calc_grid(&robots, grid_size);
    println!("Intial grid:");
    print_grid(&grid);
    let mut xmas_tree_time = 0;
    'outer: for time in 1..10000 {
        robots = robots.iter().map(|r| step(r, grid_size)).collect();
        grid = calc_grid(&robots, grid_size);

        // look for 10 non-zero entries in a row
        for window in grid.iter().collect::<Vec<&usize>>().windows(10) {
            if window.iter().all(|&&x| x != 0) {
                xmas_tree_time = time;
                break 'outer;
            }
        }
    }
    println!("\nFinal grid:");
    print_grid(&grid);
    Ok(xmas_tree_time)
}

#[test]
fn test_part1() {
    assert_eq!(part1("./input/day14_test.txt", (11, 7)).unwrap(), 12);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day14_test.txt", (11, 7)).unwrap(), 0);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile, (101, 103))?, part2(inputfile, (101, 103))?))
}
