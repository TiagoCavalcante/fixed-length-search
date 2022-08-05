use crate::graph::Graph;

/// Returns whether `vertex` is in the path to the `to`
/// vertex given the `predecessor` vector.
fn in_start_path(
  predecessor: &Vec<usize>,
  to: usize,
  vertex: usize,
) -> bool {
  let mut current = to;
  while predecessor[current] != usize::MAX {
    current = predecessor[current];
    if current == vertex {
      return true;
    }
  }
  return false;
}

/// Returns whether `vertex` is in the path to the `to`
/// vertex given the `predecessor` vector of vectors.
fn in_end_path(
  predecessor: &Vec<Vec<usize>>,
  to: usize,
  vertex: usize,
) -> bool {
  predecessor[to].iter().any(|&v| v == vertex)
}

/// Returns whether both paths share a vertex.
/// `predecessor_list` is a vector with only the
/// predecessors of a given node, while `predecessors` is
/// a predecessor vector of all vertices in the graph, so
/// we need to iterate over it to find a path.
/// We start in iterating over `predecessors` in `start`.
fn shared_paths(
  predecessor_list: &Vec<usize>,
  predecessors: &Vec<usize>,
  current: usize,
) -> bool {
  let mut current = current;

  while predecessors[current] != usize::MAX {
    current = predecessors[current];
    if predecessor_list.iter().any(|&vertex| vertex==current) {
      return true;
    }
  }

  return false;
}

/// Fixed length search algorithm.
/// For understanding this algorithm I recommend you to
/// study first how the BFS algorithm works.
/// See https://en.wikipedia.org/wiki/Breadth-first_search
/// ```
/// let path =
///   path::fixed_length_search(&graph, start, end, length);
/// println!("{:?}", path.unwrap_or(vec![]));
/// ```
pub fn fixed_length_search(
  graph: &Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  let distance = length - 1;

  // Predecessor vector as in a normal BFS algorithm.
  let mut predecessor_from_start =
    vec![usize::MAX; graph.size];
  // Distance vector as in a normal BFS algorithm.
  let mut distance_to_start = vec![usize::MAX; graph.size];

  // Differently from the BFS algorithm we need to keep the
  // distances to both the start and the end.
  // Also differently from the BFS algorithm we save the
  // predeecessors of each vertex in its own array, this is
  // necessary to avoid loops in the graph.
  // Also this allow us to keep the distance as the length
  // of the predecessor array.
  let mut predecessor_from_end = vec![vec![]; graph.size];

  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // The distance from the start to itself is 0.
  distance_to_start[start] = 0;
  queue.push_front(start);

  // [Almost] Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  // Note that in the BFS algorithm the queue must be
  // first in first out.
  while let Some(current) = queue.pop_front() {
    // Possible optimization for graphs where all vertex are
    // reachable from the start: keep count on how many
    // vertices were visited and stop once that number is
    // equal to the total number of vertices.
    for &vertex in graph.get_neighbors(current) {
      // If the distance is usize::MAX then that vertex was
      // never reached before.
      if distance_to_start[vertex] == usize::MAX {
        distance_to_start[vertex] =
          distance_to_start[current] + 1;
        predecessor_from_start[vertex] = current;
        // In a normal BFS algorithm we would stop if
        // vertex is the end, but in the fixed length search
        // we need to know the distance to each vertex from
        // the start.
        queue.push_back(vertex);
      }
    }
  }

  // Return early if this node can't be reached or if its
  // shortest path length is bigger than the desired length.
  // Note that we don't need to directly check if
  // distance_to_start[end] == usize::MAX because if it is
  // equal to usize::MAX then it is bigger than thedistance.
  if distance_to_start[end] > distance {
    return None;
  }

  // Here we are starting from the end and going to the
  // start.
  queue.push_front(end);

  // Here the magic happens.
  // Instead of finding the smallest path we are trying to
  // find the biggest path that is no bigger than the
  // length.
  // We want it to be exactly equal to the length, but we
  // won't get there so easy.
  // Contrary to BFS, here the queue must be first last out
  // out, otherwise it could (and that almost always happen)
  // change the path to a vertex without updating its
  // distance, so when it finds a path with the correct
  // length, the predecessor array would have changed and
  // a path with a bigger length would be returned instead.
  while let Some(current) = queue.pop_front() {
    for &neighbor in graph.get_neighbors(current) {
      // If we never visited this vertex or the size of the
      // path is bigger than the last path but still not
      // bigger than the length and that neighbor is not in
      // the path to the current vertex.
      // Note: if the vertex has no predecessors then it
      // was never reached.
      if (predecessor_from_end[neighbor].len() == 0
        || (predecessor_from_end[current].len() + 1
          > predecessor_from_end[neighbor].len()
          // If the sum of both is less than length, then
          // their sum + 1 won't be bigger than length.
          && predecessor_from_end[current].len()
            + distance_to_start[neighbor]
            < distance))
        // If it is already in path then we won't go to
        // this neighbor as we can't use any vertex more
        // than once.
        && !in_end_path(&predecessor_from_end, current, neighbor)
        // The contrary may also happen.
        && !in_start_path(&predecessor_from_start, neighbor, current)
        // This is the slowest test, but if we remove this
        // the algorithm may fail in small graphs.
        // Possible optimization: Move the above check to
        // inside the shared_paths function.
        && !shared_paths(&predecessor_from_end[current],&predecessor_from_start, neighbor)
      {
        predecessor_from_end[neighbor].clear();
        let current_path =
          predecessor_from_end[current].clone();
        predecessor_from_end[neighbor].extend(current_path);
        predecessor_from_end[neighbor].push(current);

        if distance_to_start[neighbor]
          + predecessor_from_end[neighbor].len()
          == distance
        {
          // First find the path between the end and the
          // current vertex.
          let mut path =
            predecessor_from_end[neighbor].clone();

          // Then append the path between the current vertex
          // and the start.
          let mut current = neighbor;

          path.push(current);

          while predecessor_from_start[current]
            != usize::MAX
          {
            current = predecessor_from_start[current];
            path.push(current);
          }

          // And then reverse the path.
          path.reverse();

          return Some(path);
        }

        queue.push_back(neighbor);
      }
    }
  }

  return None;
}
