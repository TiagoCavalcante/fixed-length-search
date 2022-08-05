use crate::rand::BoolRng;

pub struct Graph {
  pub size: usize,
  data: Vec<Vec<usize>>,
}

impl Graph {
  pub fn add_edge(&mut self, a: usize, b: usize) {
    self.data[a].push(b);
    self.data[b].push(a);
  }

  pub fn remove_edge(&mut self, a: usize, b: usize) {
    let b_position =
      self.data[a].iter().position(|v| *v == b).unwrap();
    // Remove b from a.
    self.data[a].swap_remove(b_position);

    // Remove a from b.
    let a_position =
      self.data[b].iter().position(|v| *v == a).unwrap();
    self.data[b].swap_remove(a_position);
  }

  pub fn has_edge(&self, a: usize, b: usize) -> bool {
    self.data[a].iter().any(|v| *v == b)
  }

  pub fn get_neighbors(
    &self,
    vertex: usize,
  ) -> &Vec<usize> {
    &self.data[vertex]
  }

  /// Returns the neighbors of `vertex` and remove the edges
  /// between `vertex` and its neighbors.
  pub fn pop_edges(&mut self, vertex: usize) -> Vec<usize> {
    let neighbors = self.data[vertex].clone();

    for neighbor in &neighbors {
      let position = self.data[*neighbor]
        .iter()
        .position(|v| *v == vertex)
        .unwrap();

      self.data[*neighbor].swap_remove(position);
    }

    self.data[vertex].clear();

    neighbors
  }

  /// Add edges between `vertex` and each neighbor of
  /// `neighbors`, it is usually used in conjunction with
  /// `pop_edges`.
  /// ```
  /// let neighbors = graph.pop_edges(vertex);
  /// let path_without_vertex =
  ///   path::shortest_path(&graph, a, b);
  /// // Restore the edges.
  /// graph.add_edges(vertex, neighbors);
  /// ```
  pub fn add_edges(
    &mut self,
    vertex: usize,
    neighbors: &Vec<usize>,
  ) {
    for neighbor in neighbors {
      self.add_edge(vertex, *neighbor);
    }
  }

  fn max_data_density(&self) -> f32 {
    (self.size as f32 - 1.0) / self.size as f32
  }

  pub fn fill(&mut self, density: f32) {
    let mut bool_rng =
      BoolRng::new(density / self.max_data_density());

    for i in 0..self.size {
      for j in 0..self.size {
        // If i > j it already was added.
        if i < j {
          if bool_rng.sample() {
            self.add_edge(i, j);
          }
        }
      }
    }
  }

  pub fn new(size: usize) -> Graph {
    Graph {
      size,
      data: vec![vec![]; size],
    }
  }
}
