mod graph;
mod path;
mod rand;

use std::time::Instant;

fn main() {
  let size = 10_000;
  let start = 0;
  let end = 17;
  let length = 11;

  let mut graph = graph::Graph::new(size);

  let now = Instant::now();
  graph.fill(0.1);
  println!("Fill the graph - {:.2?}", now.elapsed());

  let now = Instant::now();
  let path =
    path::fixed_length_search(&graph, start, end, length);
  println!("Fixed length search - {:.2?}", now.elapsed());

  // Test if the path is valid.
  if let Some(path) = path {
    assert_eq!(path.len(), length);
    assert_eq!(*path.first().unwrap(), start);
    assert_eq!(*path.last().unwrap(), end);

    // Check if the path is made only by real edges.
    for index in 0..path.len() - 1 {
      assert!(graph.has_edge(path[index], path[index + 1]));
    }

    // Ensure that the path contain no loops.
    let mut unique = path.clone();
    // We need a sorted vector to use dedup.
    unique.sort();
    unique.dedup();
    // If the path had loops then the length of the unique
    // vector would be smaller than the length of the path.
    assert_eq!(path.len(), unique.len());

    println!("The path is valid");
  } else {
    panic!("Couldn't find a valid path")
  }
}
