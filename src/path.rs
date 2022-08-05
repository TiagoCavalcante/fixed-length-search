use crate::graph::Graph;

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

pub fn fixed_length_bfs(
  graph: &Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  let distance = length - 1;

  let mut predecessor_from_start =
    vec![usize::MAX; graph.size];
  let mut distance_to_start = vec![usize::MAX; graph.size];

  let mut predecessor_from_end =
    vec![usize::MAX; graph.size];
  let mut distance_to_end = vec![usize::MAX; graph.size];

  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // The distance from the start to itself is 0.
  distance_to_start[start] = 0;
  queue.push_front(start);

  // Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  // Note that in the BFS algorithm the queue must be
  // first in last out.
  while let Some(current) = queue.pop_front() {
    for vertex in graph.get_neighbors(current) {
      // If the distance is usize::MAX then that vertex was
      // never reached before.
      if distance_to_start[*vertex] == usize::MAX {
        distance_to_start[*vertex] =
          distance_to_start[current] + 1;
        predecessor_from_start[*vertex] = current;
        queue.push_back(*vertex);
      }
    }
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
  // Contrary to BFS, here the queue must be first in first
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
            < distance
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
          == distance
        {
          // First find the path between the first vertex
          // and the current.
          let mut path = vec![current];
          let mut current = *vertex;

          while predecessor_from_start[current]
            != usize::MAX
          {
            path.push(predecessor_from_start[current]);
            current = predecessor_from_start[current];
          }

          path.reverse();

          // Then append the path between the current vertex
          // and the last.
          current = *vertex;

          while predecessor_from_end[current] != usize::MAX
          {
            path.push(predecessor_from_end[current]);
            current = predecessor_from_end[current];
          }

          return Some(path);
        }

        queue.push_back(*vertex);
      }
    }
  }

  return None;
}

/// Implementation of the Breadth First Search.
/// See https://en.wikipedia.org/wiki/Breadth-first_search
/// This scans the graph level by level, passing through
/// each vertex at most once.
/// It stops when the end vertex is reached.
/// The distances to each vertex from the start are stored
/// in the `distance` vector, that is initialized to
/// `usize::MAX`.
/// The predecessors of each vertex are stored in the
/// `predecessor` vector, that is also initilized to
/// `usize::MAX`, so if the predecessor of a node is
/// `usize::MAX` then that node was never reached.
/// This modifies the `distance` and `predecessor` vectors.
/// Returns `true` if `start` and `end` are connected and
/// `false` otherwise.
fn bfs(
  graph: &Graph,
  start: usize,
  end: usize,
  predecessor: &mut Vec<usize>,
) -> bool {
  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // Here usize::MAX is used to indicate infinite distance.
  let mut distance = vec![usize::MAX; graph.size];

  // The distance from the start to itself is 0.
  distance[start] = 0;
  queue.push_back(start);

  // Standard BFS algorithm.
  while let Some(current) = queue.pop_front() {
    for vertex in graph.get_neighbors(current) {
      // If it wasn't visited.
      if distance[*vertex] == usize::MAX {
        distance[*vertex] = distance[current] + 1;
        predecessor[*vertex] = current;
        queue.push_back(*vertex);

        // We stop the BFS when we find the destination.
        if *vertex == end {
          return true;
        }
      }
    }
  }

  return false;
}

/// Returns the shortest path between `start` and `end`.
/// Returns `None` if no path exists.
/// ```
/// let graph = graph::Graph::new(300, 0.01);
/// let path =
///   path::shortest_path(&graph, 0, 299).unwrap_or(vec![]);
/// println!("{:?}", path);
/// ```
pub fn shortest_path(
  graph: &Graph,
  start: usize,
  end: usize,
) -> Option<Vec<usize>> {
  // Here usize::MAX is used to indicate that there is no
  // predecessor.
  let mut predecessor = vec![usize::MAX; graph.size];

  if bfs(graph, start, end, &mut predecessor) {
    let mut path = vec![end];
    let mut current = end;
    while predecessor[current] != usize::MAX {
      path.push(predecessor[current]);
      current = predecessor[current];
    }

    path.reverse();

    Some(path)
  } else {
    // Source and destination are not connected.
    None
  }
}

fn equal_paths(a: &Vec<usize>, b: &Vec<usize>) -> bool {
  a.iter().zip(b).all(|(a, b)| *a == *b)
}

/// Use Yen algorithm for find a path with length `length`.
/// Returns None if this path doesn't exist.
pub fn yen(
  graph: &mut Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  if let Some(shortest) = shortest_path(&graph, start, end)
  {
    let mut paths = vec![shortest];
    let mut candidates: Vec<Vec<usize>> = vec![];

    for k in 1..=graph.size - length {
      // The spur node ranges from the first node to the
      // next to last node in the previous k-shortest path.
      let last_length = paths[k - 1].len();

      for i in 0..last_length - 1 {
        // Spur node is retrieved from the previous
        // k-shortest path, k − 1.
        let spur_node = paths[k - 1][i];
        // The sequence of nodes from the source to the spur
        // node of the previous k-shortest path.
        let root_path = paths[k - 1][0..i].to_vec();

        let mut edges = vec![];
        let mut nodes = vec![];

        for p in paths.iter() {
          if p.len() > i + 1
            && equal_paths(&root_path, &p[0..i].to_vec())
          {
            // Remove the links that are part of the
            // previous shortest paths which share
            // the same root path.
            if graph.has_edge(p[i], p[i + 1]) {
              graph.remove_edge(p[i], p[i + 1]);
              edges.push((p[i], p[i + 1]));
            }
          }
        }

        for node in &root_path {
          if *node != spur_node {
            nodes.push(graph.pop_edges(*node));
          }
        }

        // Calculate the spur path from the spur node to the
        // end.
        // Consider also checking if any spur_path found.
        if let Some(spur_path) =
          shortest_path(graph, spur_node, end)
        {
          // Entire path is made up of the root path and
          // spur path.
          let mut total_path = root_path.clone();
          total_path.extend(spur_path);

          // Add the potential k-shortest path to the heap.
          if candidates
            .iter()
            .find(|path| equal_paths(path, &total_path))
            .is_none()
          {
            candidates.push(total_path);
          }

          // Add back the edges and nodes that were removed
          // from the graph.
          for (a, b) in edges {
            graph.add_edge(a, b);
          }

          for (node, neighbors) in nodes.iter().enumerate()
          {
            graph.add_edges(node, neighbors)
          }
        }
      }

      if let Some((index, shortest)) = candidates
        .iter()
        .enumerate()
        .min_by_key(|(_, path)| path.len())
      {
        if shortest.len() == length {
          return Some(shortest.clone());
        }

        if shortest.len() > length {
          // If the last path was smaller than length and the
          // current path is greater than length, then no
          // paths with the desired length exist.
          return None;
        }

        // Add the lowest cost path becomes the k-shortest
        // path.
        paths.push(shortest.clone());
        candidates.swap_remove(index);
      } else {
        // This handles the case of there being no spur
        // paths, or no spur paths left.
        // This could happen if the spur paths have already
        // been exhausted (added to paths),
        // or there are no spur paths at all - such as when
        // both the source and sink vertices
        // lie along a "dead end".
        break;
      }
    }

    None
  } else {
    None
  }
}
