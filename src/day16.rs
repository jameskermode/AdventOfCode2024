use anyhow::Result;
use grid::Grid;
use petgraph::algo::{astar, dijkstra};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{EdgeRef, NodeRef};
use petgraph::Directed;
use std::collections::HashMap;
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

fn print_map(
    grid: &Grid<char>,
    graph: &Graph<(usize, usize, Option<Dir>), Dir, Directed>,
    path: &Vec<NodeIndex>,
) {
    let mut grid = grid.clone();
    for step in path {
        let (r, c, dir) = graph[*step];
        grid[(r, c)] = match dir {
            Some(North) => '^',
            Some(East) => '>',
            Some(South) => 'v',
            Some(West) => '<',
            None => '.',
        }
    }
    for rowidx in 0..grid.rows() {
        let rowstr: String = grid.iter_row(rowidx).collect();
        println!("{}", rowstr);
    }
}

fn part1(inputfile: &str) -> Result<i32> {
    let grid = read_input(inputfile)?;
    let mut graph = Graph::<(usize, usize, Option<Dir>), Dir, Directed>::new();
    let mut start_node: Option<_> = None;
    let mut end_node: Option<_> = None;
    let mut nodes: HashMap<(usize, usize), _> = HashMap::new();
    for ((r, c), &val) in grid.indexed_iter() {
        let node = graph.add_node((r, c, None));
        nodes.insert((r, c), node);
        match val {
            '#' => continue,
            'S' => start_node = Some(node),
            'E' => end_node = Some(node),
            _ => (),
        }
    }

    for ((r, c), &val) in grid.indexed_iter() {
        if val == '#' {
            continue;
        }
        let n = nodes[&(r, c)];
        let r: i32 = r as i32;
        let c: i32 = c as i32;
        for (dir, (i, j)) in zip(
            [South, North, East, West],
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)],
        ) {
            let i = i as usize;
            let j = j as usize;
            if let Some('.') | Some('S') | Some('E') = grid.get(i, j) {
                let m = nodes[&(i, j)];
                graph.add_edge(n, m, dir);
            }
        }
    }

    // println!("{:#?}", graph);
    let mut dir_graph = graph.clone();
    dir_graph[start_node.unwrap()].2 = Some(East);
    let Some((score, path)) = astar(
        &graph,
        start_node.unwrap(),
        |n| n == end_node.unwrap(),
        |edge| {
            let new_dir = *edge.weight();
            println!("edge {:?}", edge);
            // println!("prev_directions {:?}", prev_directions);
            let current_dir = dir_graph[edge.source()].2.unwrap();
            println!("{:?}", current_dir);
            let mut cost = 1;
            if current_dir != new_dir {
                cost += 1000;
            }
            for e in graph.edges(edge.target()) {
                // if e == edge {
                //     continue;
                // }
                dir_graph[e.source()].2 = Some(new_dir);
                // prev_directions.insert(e.source(), new_dir);
            }
            cost
        },
        |_| 0,
    ) else {
        panic!("no path found")
    };
    println!("{:#?}", path);
    print_map(&grid, &dir_graph, &path);
    // println!("{:?}, {}", end_node.unwrap(), path[&end_node.unwrap()]);
    Ok(score) //path[&end_node.unwrap()])
}

fn part2(inputfile: &str) -> Result<i32> {
    // let ... = read_input(inputfile)?;
    // Ok(...)
    // todo!()
    Ok(0)
}

#[test]
fn test_part1_1() {
    assert_eq!(part1("./input/day16_test.txt").unwrap(), 7036);
}

#[test]
fn test_part1_2() {
    assert_eq!(part1("./input/day16_test_2.txt").unwrap(), 11048);
}

#[test]
fn test_part2() {
    assert_eq!(part2("./input/day16_test.txt").unwrap(), 0);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)?, part2(inputfile)?))
}
