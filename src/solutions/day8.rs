use std::collections::HashSet;

use crate::util::array::AsArray;

use super::Solver;

type Id = [char; 3];

enum Dir {
    Left,
    Right,
}

struct Node {
    id: Id,
    left: Id,
    right: Id,
}

pub struct Day8 {
    dirs: Vec<Dir>,
    nodes: Vec<Node>,
}

struct GraphNode {
    id: Id,
    left: usize,
    right: usize,
}

struct GraphWalker<'a> {
    nodes: &'a [GraphNode],
    dirs: &'a [Dir],

    current: usize,
    steps: u32,
    dir_index: usize,
}

const INPUT: &str = include_str!("../../input/day8");

fn node_index_by_id(nodes: &[Node], id: &Id) -> Option<usize> {
    nodes
        .iter()
        .enumerate()
        .find_map(|(index, other)| (id == &other.id).then_some(index))
}

fn graph_nodes(nodes: &[Node]) -> Vec<GraphNode> {
    nodes
        .iter()
        .map(|node| {
            let id = node.id;
            let left = node_index_by_id(nodes, &node.left).unwrap();
            let right = node_index_by_id(nodes, &node.right).unwrap();
            GraphNode { id, left, right }
        })
        .collect()
}

impl<'a> GraphWalker<'a> {
    fn new(nodes: &'a [GraphNode], dirs: &'a [Dir], start: usize) -> Self {
        GraphWalker {
            nodes,
            dirs,
            current: start,
            steps: 0,
            dir_index: 0,
        }
    }

    fn current_node(&self) -> &GraphNode {
        &self.nodes[self.current]
    }

    fn walk_once(&mut self) {
        let next = match self.dirs[self.dir_index] {
            Dir::Left => self.nodes[self.current].left,
            Dir::Right => self.nodes[self.current].right,
        };
        self.steps += 1;

        self.dir_index += 1;
        if self.dir_index >= self.dirs.len() {
            self.dir_index = 0;
        }

        self.current = next;
    }
}

impl Day8 {
    #[allow(dead_code)]
    fn examine_data(&self) {
        // This function shows that every starting point (A) has one distinct ending point (Z) and a distinct loop length until
        // it repeats its path along the graph. By making this assumption, part 2 becomes easier to solve.

        let graph_nodes = graph_nodes(&self.nodes);

        let starts: Vec<(usize, &GraphNode)> = graph_nodes.iter().enumerate().filter(|(_, node)| node.id[2] == 'A').collect();

        for (index, start) in starts {
            let mut memo: HashSet<(usize, usize)> = HashSet::new();
            let mut zs: HashSet<Id> = HashSet::new();
            let mut walker = GraphWalker::new(&graph_nodes, &self.dirs, index);

            while !memo.contains(&(walker.current, walker.dir_index)) {
                memo.insert((walker.current, walker.dir_index));
                if walker.current_node().id[2] == 'Z' {
                    zs.insert(walker.current_node().id);
                }
                walker.walk_once();
            }
            println!(
                "Starting at {:?} loops after {} steps and has {} Zs: {:?}",
                start.id,
                walker.steps,
                zs.len(),
                zs.iter().collect::<Vec<&Id>>()
            );
        }
    }
}

impl Solver for Day8 {
    type Solution1 = u32;
    type Solution2 = u64;

    fn new() -> Self {
        Day8 {
            dirs: vec![],
            nodes: vec![],
        }
    }

    fn reset(&mut self) {
        self.nodes.clear();
    }

    fn parse_input(&mut self) {
        let (dirs, nodes) = INPUT.split_once("\n\n").unwrap();
        self.dirs = dirs
            .chars()
            .map(|ch| match ch {
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => panic!(),
            })
            .collect();
        self.nodes = nodes
            .lines()
            .map(|line| {
                let (id, children) = line.split_once('=').unwrap();
                let id = id.trim().as_array();
                let children = children.trim();
                let children = &children[1..children.len() - 1];
                let (left, right) = children.split_once(", ").unwrap();
                let left = left.as_array();
                let right = right.as_array();
                Node { id, left, right }
            })
            .collect();
    }

    fn solve_part1(&self) -> u32 {
        let graph_nodes = graph_nodes(&self.nodes);

        let aaa = node_index_by_id(&self.nodes, &['A', 'A', 'A']).unwrap();
        let zzz = node_index_by_id(&self.nodes, &['Z', 'Z', 'Z']).unwrap();

        let mut walker = GraphWalker::new(&graph_nodes, &self.dirs, aaa);
        while walker.current != zzz {
            walker.walk_once();
        }

        walker.steps
    }

    fn solve_part2(&self) -> u64 {
        let graph_nodes = graph_nodes(&self.nodes);

        graph_nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| (node.id[2] == 'A').then_some(index))
            .map(|start| {
                let mut memo: HashSet<(usize, usize)> = HashSet::new();
                let mut z = None;
                let mut walker = GraphWalker::new(&graph_nodes, &self.dirs, start);

                while !memo.contains(&(walker.current, walker.dir_index)) {
                    memo.insert((walker.current, walker.dir_index));
                    if walker.current_node().id[2] == 'Z' {
                        z = Some(walker.steps);
                    }
                    walker.walk_once();
                }

                let z = z.unwrap();
                let loop_cycles = walker.steps as usize / self.dirs.len();
                if z as usize != loop_cycles * self.dirs.len() {
                    panic!();
                }
                loop_cycles as u64
            })
            .product::<u64>()
            * self.dirs.len() as u64
    }

    fn print_solutions(&self, part1: u32, part2: u64) {
        println!("Steps to reach ZZZ: {part1}");
        println!("Steps for all nodes to reach Z simultaneously: {part2}");
    }
}
