use anyhow::Result;
use grid::Grid;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct State {
    grid: Grid<char>,
    moves: Vec<Dir>,
}

fn read_input(inputfile: &str) -> Result<State> {
    let data = fs::read_to_string(inputfile)?;
    let (grid_data, move_data) = data.split_once("\n\n").unwrap();

    let chars: Vec<_> = grid_data.chars().filter(|&c| c != '\n').collect();
    let grid = Grid::from_vec(chars, data.lines().next().unwrap().len());

    let moves: Vec<_> = move_data
        .chars()
        .filter_map(|ch| match ch {
            '^' => Some(Dir::Up),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            '>' => Some(Dir::Right),
            '\n' => None,
            _m => panic!("bad move {}", _m),
        })
        .collect();
    Ok(State { grid, moves })
}

fn print_grid(grid: &Grid<char>) {
    // print!("{}[2J", 27 as char);
    for rowidx in 0..grid.rows() {
        let rowstr: String = grid.iter_row(rowidx).collect();
        println!("{rowstr}");
    }
}

fn step(grid: &mut Grid<char>, dir: Dir, moves_dict: &HashMap<Dir, (i32, i32)>, part2: bool) {
    let (r, c) = grid
        .indexed_iter()
        .filter_map(|((row, col), &val)| if val == '@' { Some((row, col)) } else { None })
        .next()
        .unwrap();
    let (dr, dc) = moves_dict[&dir];

    let rn = (r as i32 + dr) as usize;
    let cn = (c as i32 + dc) as usize;
    match grid.get(rn, cn) {
        Some('.') => {
            grid[(rn, cn)] = '@';
            grid[(r, c)] = '.';
        }
        Some('#') => (),
        Some('O') => {
            assert!(!part2);
            let mut rb = rn as i32;
            let mut cb = cn as i32;
            let mut block_indices: Vec<(usize, usize)> = vec![];
            while let Some('O') = grid.get(rb as usize, cb as usize) {
                rb += dr;
                cb += dc;
                block_indices.push((rb as usize, cb as usize));
            }
            if let Some('.') = grid.get(rb, cb) {
                grid[(r, c)] = '.';
                for block_index in block_indices {
                    grid[block_index] = 'O';
                }
                grid[(rn, cn)] = '@';
            }
        }
        Some('[') | Some(']') => {
            assert!(part2);
            let mut rb = rn as i32;
            let mut cb = cn as i32;
            let mut block_indices: Vec<(usize, usize)> = vec![];
            let blocks = [
                grid[(rn, cn)],
                if grid[(rn, cn)] == '[' { ']' } else { '[' },
            ];
            while let Some('[') | Some(']') = grid.get(rb as usize, cb as usize) {
                rb += dr;
                cb += dc;
                block_indices.push((rb as usize, cb as usize));
            }
            if let Some('.') = grid.get(rb, cb) {
                grid[(r, c)] = '.';
                for (count, &block_index) in block_indices.iter().enumerate() {
                    grid[block_index] = blocks[count % 2];
                }
                grid[(rn, cn)] = '@';
            }
        }
        Some(ch) => panic!("bad grid entry {} at row {} col {}", ch, r, c),
        None => (),
    }
}

fn score(grid: &Grid<char>) -> usize {
    grid.indexed_iter()
        .filter_map(|((r, c), &val)| (val == 'O').then_some(100 * r + c))
        .sum()
}

fn make_moves_dict() -> HashMap<Dir, (i32, i32)> {
    let mut moves_dict = HashMap::new();
    moves_dict.insert(Dir::Up, (-1, 0));
    moves_dict.insert(Dir::Down, (1, 0));
    moves_dict.insert(Dir::Left, (0, -1));
    moves_dict.insert(Dir::Right, (0, 1));
    moves_dict
}

fn part1(inputfile: &str, verbose: bool) -> Result<i32> {
    let mut state = read_input(inputfile)?;
    if verbose {
        println!("Initial state");
        print_grid(&state.grid);
    }
    let moves_dict = make_moves_dict();

    for dir in state.moves {
        if verbose {
            println!("Move {:?}", dir)
        };
        step(&mut state.grid, dir, &moves_dict, false);
        if verbose {
            print_grid(&state.grid)
        };
    }
    println!("{}", score(&state.grid));
    Ok(score(&state.grid) as i32)
}

fn expand_grid(grid: &Grid<char>) -> Grid<char> {
    let mut big_grid = Grid::<char>::new(grid.rows(), 2 * grid.cols());
    for ((r, c), &val) in grid.indexed_iter() {
        let chars: Vec<_> = match val {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            _ => panic!("unexpected map character {}", val),
        }
        .chars()
        .collect();
        big_grid[(r, 2 * c)] = chars[0];
        big_grid[(r, 2 * c + 1)] = chars[1];
    }
    big_grid
}

fn part2(inputfile: &str, verbose: bool) -> Result<i32> {
    let state = read_input(inputfile)?;
    let mut big_grid = expand_grid(&state.grid);
    if verbose {
        println!("Initial state");
        print_grid(&big_grid);
    }
    let moves_dict = make_moves_dict();

    for dir in state.moves {
        if verbose {
            println!("Move {:?}", dir)
        };
        step(&mut big_grid, dir, &moves_dict, true);
        if verbose {
            print_grid(&big_grid)
        };
    }
    println!("{}", score(&big_grid));
    Ok(score(&big_grid) as i32)
}

#[test]
fn test_part1_0() {
    assert_eq!(part1("./input/day15_test_0.txt", true).unwrap(), 2028);
}

#[test]
fn test_part1_1() {
    assert_eq!(part1("./input/day15_test.txt", true).unwrap(), 10092);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day15_test_part2.txt", true).unwrap(), 0);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile, false)?, part2(inputfile, false)?))
}
