use anyhow::Result;
use grid::Grid;
use petgraph::algo::dijkstra;
use petgraph::graph::Graph;
use petgraph::Directed;
use std::fs;
use std::iter::zip;

fn read_input(inputfile: &str) -> Result<Grid<char>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data.chars().filter(|&c| c != '\n').collect();
    let grid = Grid::from_vec(chars, data.lines().next().unwrap().len());
    Ok(grid)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}
use Dir::*;

fn part1(inputfile: &str) -> Result<i32> {
    let grid = read_input(inputfile)?;
    let mut graph = Graph::<(usize, usize), Dir, Directed>::new();
    let mut start_node: Option<_> = None;
    let mut end_node: Option<_> = None;
    let mut nodes: Vec<_> = vec![];
    for ((r, c), &val) in grid.indexed_iter() {
        nodes.push(graph.add_node((r, c)));
        match val {
            '#' => continue,
            'S' => start_node = Some(nodes.last().unwrap().clone()),
            'E' => end_node = Some(nodes.last().unwrap().clone()),
            _ => (),
        }
    }

    for ((r, c), &val) in grid.indexed_iter() {
        if val == '#' {
            continue;
        }
        let n = nodes.iter().position(|&idx| graph[idx] == (r, c)).unwrap();
        let r: i32 = r as i32;
        let c: i32 = c as i32;
        for (dir, (i, j)) in zip(
            [North, South, West, East],
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)],
        ) {
            let i = i as usize;
            let j = j as usize;
            if let Some('.') | Some('S') | Some('E') = grid.get(i, j) {
                let m = nodes.iter().position(|&idx| graph[idx] == (i, j)).unwrap();
                graph.add_edge(nodes[n], nodes[m], dir);
            }
        }
    }
    // println!("{:#?}", graph);
    let mut current_dir = East;
    let path = dijkstra(&graph, start_node.unwrap(), end_node, |edge| {
        let new_dir = *edge.weight();
        let cost = match current_dir {
            North => match new_dir {
                North => 1,
                East => 1000,
                West => 1000,
                South => 2000,
            },
            East => match new_dir {
                East => 1,
                North => 1000,
                South => 1000,
                West => 2000,
            },
            South => match new_dir {
                South => 1,
                East => 1000,
                West => 1000,
                North => 2000,
            },
            West => match new_dir {
                West => 1,
                North => 1000,
                South => 1000,
                East => 2000,
            },
        };
        current_dir = new_dir;
        cost
    });
    println!("{:#?}", path);
    println!("{:?}, {}", end_node.unwrap(), path[&end_node.unwrap()]);
    Ok(path[&end_node.unwrap()])
}

fn part2(inputfile: &str) -> Result<i32> {
    // let ... = read_input(inputfile)?;
    // Ok(...)
    todo!()
}

#[test]
fn test_part1_1() {
    assert_eq!(part1("./input/day16_test.txt").unwrap(), 7036);
}

#[test]
fn test_part1_2() {
    assert_eq!(part1("./input/day16_test_2.txt").unwrap(), 11);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day16_test.txt").unwrap(), 0);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
