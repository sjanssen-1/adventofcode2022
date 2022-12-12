use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i32,
    position: usize,
}

// gestolen van https://stackoverflow.com/questions/39949939/how-can-i-implement-a-min-heap-of-f64-with-rusts-binaryheaps
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: i32,
}

// Dijkstra's shortest path algorithm.
// variant on pseudo as found on wikipedia:
// 1  function Dijkstra(Graph, source):
// 2      dist[source] ← 0                           // Initialization
// 3
// 4      create vertex priority queue Q
// 5
// 6      for each vertex v in Graph.Vertices:
// 7          if v ≠ source
// 8              dist[v] ← INFINITY                 // Unknown distance from source to v
// 9              prev[v] ← UNDEFINED                // Predecessor of v
// 10
// 11         Q.add_with_priority(v, dist[v])
// 12
// 13
// 14     while Q is not empty:                      // The main loop
// 15         u ← Q.extract_min()                    // Remove and return best vertex
// 16         for each neighbor v of u:              // Go through all v neighbors of u
// 17             alt ← dist[u] + Graph.Edges(u, v)
// 18             if alt < dist[v]:
// 19                 dist[v] ← alt
// 20                 prev[v] ← u
// 21                 Q.decrease_priority(v, alt)
// 22
// 23     return dist, prev
fn shortest_path(graph: &Vec<Vec<Edge>>, start_node: usize, end_node: usize) -> Option<i32> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist= vec![i32::MAX; graph.len()];

    let mut heap = BinaryHeap::new();

    dist[start_node] = 0;
    heap.push(State { cost: 0, position: start_node });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        if position == end_node { return Some(cost); }

        if cost > dist[position] { continue; }

        for edge in &graph[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None // no path
}

fn main() {
    let landscape_input = read_to_string("data/day12_personal.txt").expect("get rekt");
    let width = landscape_input.lines().next().unwrap().len();
    let heigth = landscape_input.lines().count();
    let mut landscape = vec![vec![0; width]; heigth];

    let mut start: (usize, usize) = (0,0);
    let mut possible_starting_points: Vec<(usize, usize)> = Vec::new();
    let mut end: (usize, usize) = (0,0);

    for (row, line) in landscape_input.lines().enumerate() {
        for (column, node) in line.trim().chars().enumerate() {
            if node == 'S' {
                start = (column, row);
                possible_starting_points.push((column, row));
                landscape[row][column] = 'a' as i32;
            } else if node == 'E' {
                end = (column, row);
                landscape[row][column] = 'z' as i32;
            } else {
                if node == 'a' {
                    possible_starting_points.push((column, row));
                }
                landscape[row][column] = node as i32;
            }
        }
    }

    let mut graph: Vec<Vec<Edge>> = Vec::new();
    for row in 0..heigth {
        for column in 0..width {
            let mut edges: Vec<Edge> = Vec::new();
            if column != width-1 && landscape[row][column] - landscape[row][column+1] >= -1 {
                // right edge
                edges.push(Edge{ node: width * row + (column+1), cost: 1 });
            }
            if column != 0 && landscape[row][column] - landscape[row][column-1] >= -1{
                // left edge
                edges.push(Edge{ node: width * row + (column-1), cost: 1 });
            }
            if row != heigth-1 && landscape[row][column] - landscape[row+1][column] >= -1{
                // down edge
                edges.push(Edge{ node: width * (row+1) + column, cost: 1 });
            }
            if row != 0 && landscape[row][column] - landscape[row-1][column] >= -1{
                // up edge
                edges.push(Edge{ node: width * (row-1) + column, cost: 1 });
            }
            graph.push(edges);
        }
    }

    println!("part 1: {:?}", shortest_path(&graph, width * start.1 + start.0, width * end.1 + end.0));
    let mut multiple: Vec<Option<i32>> = possible_starting_points
        .iter()
        .map(|s| shortest_path(&graph, width * s.1 + s.0, width * end.1 + end.0) )
        .filter(|x| x.is_some())
        .collect();
    multiple.sort();
    println!("part 2: {:?}", multiple[0]);
}