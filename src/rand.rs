use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

/// Random boolean generator
/// ```
/// let mut bool_rng = BoolRng::new(0.5);
/// let is_true: bool = bool_rng.sample();
/// ```
pub struct BoolRng {
  uniform_rng: Uniform<usize>,
  rng: ThreadRng,
  threshold: usize,
}

impl BoolRng {
  /// Receives the probability of yielding `true`.
  pub fn new(probability: f32) -> BoolRng {
    let uniform_rng: Uniform<usize> =
      Uniform::from(0..usize::MAX);
    let rng: ThreadRng = rand::thread_rng();

    BoolRng {
      uniform_rng,
      rng,
      threshold: (probability * usize::MAX as f32) as usize,
    }
  }

  pub fn sample(&mut self) -> bool {
    self.uniform_rng.sample(&mut self.rng) < self.threshold
  }
}
