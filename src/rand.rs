use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// Uniform random number generator.
/// ```
/// let mut uniform_rng = UniformRng::new(1, 6);
/// let dice: usize = uniform_rng.sample();
/// ```
pub struct UniformRng {
  uniform_rng: Uniform<usize>,
  rng: ThreadRng,
}

impl UniformRng {
  pub fn new(start: usize, end: usize) -> UniformRng {
    let uniform_rng: Uniform<usize> =
      Uniform::from(start..end);
    let rng: ThreadRng = rand::thread_rng();

    UniformRng { uniform_rng, rng }
  }

  pub fn sample(&mut self) -> usize {
    self.uniform_rng.sample(&mut self.rng)
  }
}
