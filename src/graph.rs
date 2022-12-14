use crate::rand::UniformRng;

pub struct Graph {
  pub size: usize,
  data: Vec<Vec<usize>>,
}

impl Graph {
  pub fn add_edge(&mut self, a: usize, b: usize) {
    self.data[a].push(b);
    self.data[b].push(a);
  }

  pub fn has_edge(&self, a: usize, b: usize) -> bool {
    self.data[a].iter().any(|&neighbor| neighbor == b)
  }

  pub fn get_neighbors(
    &self,
    vertex: usize,
  ) -> &Vec<usize> {
    &self.data[vertex]
  }

  fn max_data_density(&self) -> f32 {
    (self.size as f32 - 1.0) / self.size as f32
  }

  pub fn fill(&mut self, density: f32) {
    let real_density = density / self.max_data_density();

    let marked = (real_density
      // This is squared because we need to "throw the coin"
      // for each pair of vertices.
      * self.size.pow(2) as f32
      // And divided by 2 because when we add a connection
      // we add 2 edges, as the graph is undirected.
      * 0.5) as usize;

    let mut vertex_rng = UniformRng::new(0, self.size);

    for _ in 0..marked {
      let a = vertex_rng.sample();
      let b = vertex_rng.sample();

      if a != b {
        self.add_edge(a, b);
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
