use petgraph::Graph;
use petgraph::Undirected;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Bfs, Walker};
use petgraph::algo::connected_components;

type Node = u32;

type Plumbing = Graph<(), (), Undirected>;


fn parse(line: &str) -> (Node, Vec<Node>) {
    let split = line.trim().split(" <-> ").collect::<Vec<&str>>();
    (
        split[0].parse().unwrap(),
        split[1].split(", ").map(|x| x.parse().unwrap()).collect(),
    )
}

fn all_pairs<'a>(input: &'a str) -> impl Iterator<Item = (Node, Node)> + 'a {
    input.lines().map(parse).flat_map(|(source, targets)| {
        targets.into_iter().map(move |target| (source, target))
    })
}

pub fn part1(input: &str) -> usize {
    let graph: Plumbing = Graph::from_edges(all_pairs(input));
    Bfs::new(&graph, NodeIndex::from(0)).iter(&graph).count()
}

pub fn part2(input: &str) -> usize {
    let graph: Plumbing = Graph::from_edges(all_pairs(input));
    connected_components(&graph)
}

#[test]
fn examples() {
    let input = "0 <-> 2
                 1 <-> 1
                 2 <-> 0, 3, 4
                 3 <-> 2, 4
                 4 <-> 2, 3, 6
                 5 <-> 6
                 6 <-> 4, 5";
    assert_eq!(6, part1(input));
    assert_eq!(2, part2(input));
}
