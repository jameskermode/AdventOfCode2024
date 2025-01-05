use anyhow::Result;
use grid::Grid;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::zip;

fn read_input(inputfile: &str) -> Result<Grid<char>> {
    let data = fs::read_to_string(inputfile)?;
    let chars: Vec<_> = data.chars().filter(|&c| c != '\n').collect();
    let grid = Grid::from_vec(chars, data.lines().next().unwrap().len());
    Ok(grid)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Dir {
    North = 0,
    East = 270,
    South = 180,
    West = 90,
}
use Dir::*;

fn angle(dir1: Dir, dir2: Dir) -> i32 {
    (dir1 as i32) - (dir2 as i32)
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Dir,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Clone, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    nodes: &Vec<(usize, usize, Dir)>,
    node_map: &HashMap<(usize, usize, Dir), usize>,
    adj_list: &Vec<Vec<Edge>>,
    start: (usize, usize),
    start_dir: Dir,
    goal: (usize, usize),
) -> Option<State> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    // We're at `start`, facing East with a zero cost
    dist[node_map[&(start.0, start.1, East)]] = 0;
    heap.push(State {
        cost: 0,
        position: start,
        direction: start_dir,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        // println!(
        //     "cost {} position {:?} direction {:?}",
        //     cost, position, direction
        // );
        let node_index = node_map[&(position.0, position.1, direction)];
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(State {
                cost,
                position,
                direction,
            });
        }

        // Important as we may have already found a better way
        if cost > dist[node_index] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[node_index] {
            let next = State {
                cost: cost + edge.cost,
                position: (nodes[edge.node].0, nodes[edge.node].1),
                direction: nodes[edge.node].2,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[edge.node] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[edge.node] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

struct Map {
    nodes: Vec<(usize, usize, Dir)>,
    node_map: HashMap<(usize, usize, Dir), usize>,
    edges: Vec<Vec<Edge>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn make_graph(grid: &Grid<char>) -> Map {
    let mut start_node: Option<(usize, usize)> = None;
    let mut end_node: Option<(usize, usize)> = None;
    let mut nodes: Vec<_> = vec![];
    let mut node_map: HashMap<(usize, usize, Dir), usize> = HashMap::new();
    let mut current_node: usize = 0;

    for ((r, c), &val) in grid.indexed_iter() {
        match val {
            'S' => start_node = Some((r, c)),
            'E' => end_node = Some((r, c)),
            _ => (),
        }
        for direction in [North, South, East, West] {
            node_map.insert((r, c, direction), current_node);
            nodes.push((r, c, direction));
            current_node += 1;
        }
    }
    let mut edges = vec![vec![]; node_map.len()];

    for ((r, c), &val) in grid.indexed_iter() {
        if val == '#' {
            continue;
        }
        for current_dir in [South, North, East, West] {
            let n = node_map[&(r, c, current_dir)];

            for (next_dir, (i, j)) in zip(
                [South, North, East, West],
                [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)],
            ) {
                if let Some('.') | Some('S') | Some('E') = grid.get(i, j) {
                    let m = node_map[&(i, j, next_dir)];
                    let c = (1 + angle(current_dir, next_dir).abs() * 1000 / 90) as usize;
                    // if (r, c) == start_node.unwrap() {
                    // println!("adding edge {n}-{m} ({r},{c})->({i},{j}) current_dir {current_dir:?} {next_dir:?}");
                    // }
                    edges[n].push(Edge { node: m, cost: c })
                }
            }
        }
    }
    Map {
        nodes,
        node_map,
        edges,
        start: start_node.unwrap(),
        end: end_node.unwrap(),
    }
}

fn part1(inputfile: &str) -> Result<usize> {
    let grid = read_input(inputfile)?;
    let map = make_graph(&grid);
    let final_state = shortest_path(
        &map.nodes,
        &map.node_map,
        &map.edges,
        map.start,
        East,
        map.end,
    )
    .unwrap();
    Ok(final_state.cost)
}

fn part2(inputfile: &str) -> Result<usize> {
    let grid = read_input(inputfile)?;
    let map = make_graph(&grid);
    let final_state = shortest_path(
        &map.nodes,
        &map.node_map,
        &map.edges,
        map.start,
        East,
        map.end,
    )
    .unwrap();
    println!("min cost {}", final_state.cost);
    let mut shortest_path_tiles = HashSet::<(usize, usize)>::new();

    for ((r, c), &val) in grid.indexed_iter() {
        if val != '.' && val != 'S' && val != 'E' {
            continue;
        }
        let state1 = shortest_path(
            &map.nodes,
            &map.node_map,
            &map.edges,
            map.start,
            East,
            (r, c),
        )
        .unwrap();
        let state2 = shortest_path(
            &map.nodes,
            &map.node_map,
            &map.edges,
            (r, c),
            final_state.direction,
            map.end,
        )
        .unwrap();
        // println!("{:?} {}", (r, c), state1.cost + state2.cost);
        if state1.cost + state2.cost <= final_state.cost {
            shortest_path_tiles.insert((r, c));
        }
    }

    for rowidx in 0..grid.rows() {
        let mut rowstr = grid.iter_row(rowidx).collect::<String>().into_bytes();
        for colidx in 0..grid.cols() {
            if shortest_path_tiles.contains(&(rowidx, colidx)) {
                rowstr[colidx] = b'O';
            }
        }
        println!("{}", String::from_utf8(rowstr).unwrap());
    }

    Ok(shortest_path_tiles.len())
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
fn test_part2_1() {
    assert_eq!(part2("./input/day16_test.txt").unwrap(), 45);
}

#[test]
fn test_part2_2() {
    assert_eq!(part2("./input/day16_test_2.txt").unwrap(), 64);
}

pub fn run(inputfile: &str) -> Result<(i32, i32)> {
    Ok((part1(inputfile)? as i32, part2(inputfile)? as i32))
}
