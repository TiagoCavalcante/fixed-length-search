use crate::graph::Graph;

/// Returns whether `vertex` is in the path to the `to`
/// vertex given the `predecessor` vector.
fn in_path(
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
  // Predecessor vector as in a normal BFS algorithm.
  let mut predecessor_from_start =
    vec![usize::MAX; graph.size];
  // Distance vector as in a normal BFS algorithm.
  let mut distance_to_start = vec![usize::MAX; graph.size];

  // Differently from the BFS algorithm we need to keep the
  // distances to both the start and the end.
  let mut predecessor_from_end =
    vec![usize::MAX; graph.size];
  let mut distance_to_end = vec![usize::MAX; graph.size];

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
    for vertex in graph.get_neighbors(current) {
      // If the distance is usize::MAX then that vertex was
      // never reached before.
      if distance_to_start[*vertex] == usize::MAX {
        distance_to_start[*vertex] =
          distance_to_start[current] + 1;
        predecessor_from_start[*vertex] = current;
        // In a normal BFS algorithm we would stop if
        // vertex is the end, but in the fixed length search
        // we need to know the distance to each vertex from
        // the start.
        queue.push_back(*vertex);
      }
    }
  }

  // Return early if this node can't be reached or if its
  // shortest path length is bigger than the desired length.
  // Note that we don't need to directly check if
  // distance_to_start[end] == usize::MAX because if it is
  // equal to usize::MAX then it is bigger than thedistance.
  if distance_to_start[end] > length {
    return None;
  }

  // Here we are starting from the end and going to the
  // start.
  // The distance from the start to itself is 0.
  distance_to_end[end] = 0;
  queue.push_back(end);

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
  while let Some(current) = queue.pop_back() {
    for vertex in graph.get_neighbors(current) {
      // If we never visited this vertex or the size of the
      // path is bigger than the last path but still not
      // bigger than the length and that neighbor is not in
      // the path to the current vertex.
      // Note: if the distance is usize::MAX then that
      // vertex was never reached before.
      if distance_to_end[*vertex] == usize::MAX
        || (distance_to_end[current] + 1
          > distance_to_end[*vertex]
          // If the sum of both is less than length, then
          // their sum + 1 won't be bigger than length.
          && distance_to_end[current]
            + distance_to_start[*vertex]
            < length
          // If it is already in path then we won't go to
          // this neighbor as we can't use any vertex more
          // than once.
          && !in_path(
            &predecessor_from_end,
            current,
            *vertex,
          ))
      {
        distance_to_end[*vertex] =
          distance_to_end[current] + 1;
        predecessor_from_end[*vertex] = current;

        if distance_to_start[*vertex]
          + distance_to_end[*vertex]
          == length
        {
          // First find the path between the first vertex
          // and the current.
          let mut path = vec![];
          let mut current = *vertex;

          while predecessor_from_start[current]
            != usize::MAX
          {
            current = predecessor_from_start[current];
            path.push(current);
          }

          path.reverse();

          // Then append the path between the current vertex
          // and the last.
          current = *vertex;

          while predecessor_from_end[current] != usize::MAX
          {
            current = predecessor_from_end[current];
            path.push(current);
          }

          return Some(path);
        }

        queue.push_back(*vertex);
      }
    }
  }

  return None;
}
