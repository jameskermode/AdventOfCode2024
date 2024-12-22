use anyhow::Result;
use grid::Grid;
use petgraph::algo::all_simple_paths;
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;
use petgraph::{Directed, Graph};
use std::collections::{HashMap, HashSet};
use std::fs;

fn read_input(inputfile: &str) -> Result<Grid<u32>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap_or(100))
        .collect();
    let grid = Grid::from_vec(chars, data.lines().next().unwrap().len());
    Ok(grid)
}

fn _print_grid(grid: &Grid<u32>) {
    // print!("{}[2J", 27 as char);
    for rowidx in 0..grid.rows() {
        let rowstr: String = grid
            .iter_row(rowidx)
            .map(|c| match c {
                100 => ".".to_string(),
                _ => c.to_string(),
            })
            .collect();
        println!("{rowstr}");
    }
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    let grid = read_input(inputfile)?;
    _print_grid(&grid);
    let mut trail_heads: Vec<_> = vec![];
    let mut nodes: HashMap<(usize, usize), _> = HashMap::new();
    let mut graph = Graph::<(usize, usize, u32), (), Directed>::new();
    for ((i, j), &val) in grid.indexed_iter() {
        let node = graph.add_node((i, j, val));
        nodes.insert((i, j), node);
        if val == 0 {
            trail_heads.push(node);
        }
    }
    for ((i1, j1), &val1) in grid.indexed_iter() {
        let n = nodes[&(i1, j1)];
        let i1 = i1 as i32;
        let j1 = j1 as i32;
        for (i2, j2) in [(i1 - 1, j1), (i1 + 1, j1), (i1, j1 - 1), (i1, j1 + 1)] {
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            if let Some(&val2) = grid.get(i2, j2) {
                if val2 == val1 + 1 {
                    let m = nodes[&(i2, j2)];
                    graph.add_edge(n, m, ());
                }
            }
        }
    }
    // println!("{:#?}", graph);
    let mut scores: Vec<usize> = vec![0; trail_heads.len()];
    let mut end_points: HashMap<NodeIndex, HashSet<NodeIndex>> = HashMap::new();

    for (idx, trail_head) in trail_heads.iter().enumerate() {
        let mut bfs = Bfs::new(&graph, *trail_head);
        while let Some(node) = bfs.next(&graph) {
            if graph[node].2 == 9 {
                scores[idx] += 1;
                end_points.entry(*trail_head).or_default().insert(node);
            }
        }
    }

    let mut ratings: Vec<usize> = vec![0; trail_heads.len()];
    for (idx, trail_head) in trail_heads.iter().enumerate() {
        for end_point in &end_points[&trail_head] {
            ratings[idx] += all_simple_paths::<Vec<_>, _>(&graph, *trail_head, *end_point, 0, None)
                .collect::<Vec<_>>()
                .len();
        }
    }

    println!("scores {:?}", scores);
    println!("ratings {:?}", ratings);
    Ok((
        scores.iter().sum::<usize>() as i32,
        ratings.iter().sum::<usize>() as i32,
    ))
}

#[test]
fn test_1() {
    assert_eq!(run("./input/day10_test.txt").unwrap(), (36, 81));
}

#[test]
fn test_2() {
    assert_eq!(run("./input/day10_small.txt").unwrap(), (2, 227));
}
