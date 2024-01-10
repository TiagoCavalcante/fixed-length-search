use graphs::Graph;

/// Returns whether `vertex` is in the path to the `to`
/// vertex given the `predecessor` vector.
fn in_start_path(
  predecessor: &[usize],
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
  false
}

/// Returns whether both paths share a vertex.
/// `predecessor_list` is a vector with only the
/// predecessors of a given node, while `predecessors` is
/// a predecessor vector of all vertices in the graph, so
/// we need to iterate over it to find a path.
/// We start in iterating over `predecessors` in `start`.
///
/// Note that it uses `current` as the 0th vertex of the
/// `predecessors`, not `predecessors[current]`.
fn shared_paths(
  predecessor_list: &[usize],
  predecessors: &[usize],
  current: usize,
) -> bool {
  let mut current = current;

  while current != usize::MAX {
    if predecessor_list
      .iter()
      .any(|&vertex| vertex == current)
    {
      return true;
    }
    current = predecessors[current];
  }

  false
}

/// Fixed length search algorithm.
/// For understanding this algorithm, I recommend you to
/// study first how the BFS algorithm works.
/// See https://en.wikipedia.org/wiki/Breadth-first_search
///
/// The idea behind this algorithm is to first find the
/// shortest path from the start to then end, and then make
/// the reverse path trying to increase its length, but
/// without exceeding the desired length, and stop when a
/// path with the desired length is reached.
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

  // Differently from the BFS algorithm, we need to keep the
  // predecessors from both the start and the end.
  // Also differently from the BFS algorithm, we save the
  // predecessors of each vertex in its own array, this is
  // necessary to avoid paths with wrong lengths because
  // another iteration has modified the predecessors of a
  // vertex.
  // Also this allows us to keep the distance as the length
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
        // In a normal BFS algorithm, we would stop if
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
  // equal to usize::MAX then it is bigger than the
  // distance.
  if distance_to_start[end] > distance {
    return None;
  }

  // Here we are starting from the end and going to the
  // start.
  queue.push_front(end);

  // Here the magic happens.
  // Instead of finding the smallest path, we are trying to
  // find the biggest path that is no bigger than the
  // length.
  // We want it to be exactly equal to the length, but we
  // won't get there so easy.
  // In the first versions of this algorithm, the queue
  // needed to be first in last out, but in the latest
  // version it doesn't need to be anymore.
  while let Some(current) = queue.pop_front() {
    for &neighbor in graph.get_neighbors(current) {
      // If we never visited this vertex or the size of the
      // path is bigger than the last path but still not
      // bigger than the length and that neighbor is not in
      // the path to the current vertex.
      // Note: if the vertex has no predecessors, then it
      // was never reached.
      if (predecessor_from_end[neighbor].is_empty()
        // If the length of the current path is greater than
        // or equal to the length of the old path, then the
        // length of the current path + 1 will be bigger
        // than the length of the old path.
        || (predecessor_from_end[current].len()
          >= predecessor_from_end[neighbor].len()
          // If the sum of both is less than length, then
          // their sum + 1 won't be bigger than length.
          && predecessor_from_end[current].len()
            + distance_to_start[neighbor]
            < distance))
        // If it is already in path, then we won't go to
        // this neighbor, as we can't use any vertex more
        // than once.
        // && !in_end_path(&predecessor_from_end, current, neighbor)
        // The check above is implicit in shared_paths.
        // The contrary may also happen.
        && !in_start_path(&predecessor_from_start, neighbor, current)
        // This is the slowest test, but if we remove this
        // the algorithm may fail in small graphs.
        // Possible optimization: Move the above check to
        // inside the shared_paths function.
        && !shared_paths(&predecessor_from_end[current],&predecessor_from_start, neighbor)
      {
        // The code below is equivalent to:
        // predecessor_from_end[neighbor] =
        //   current_path + current;
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

        // Using push_front here instead of push_back makes
        // the algorithm up to 3x faster for big lengths.
        queue.push_front(neighbor);
      }
    }
  }

  None
}
