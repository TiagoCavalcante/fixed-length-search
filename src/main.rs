mod graph;
mod path;
mod rand;

use std::time::Instant;

fn main() {
  let mut graph = graph::Graph::new(500);

  let now = Instant::now();
  graph.fill(0.01);
  println!("Fill the graph - {:.2?}", now.elapsed());

  let now = Instant::now();
  path::fixed_length_search(&graph, 0, 17, 11).unwrap();
  println!("Fixed length search - {:.2?}", now.elapsed());

  let now = Instant::now();
  path::yen(&mut graph, 0, 17, 11).unwrap();
  println!("Yen - {:.2?}", now.elapsed());
}
