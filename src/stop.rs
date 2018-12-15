use std::sync::Arc;

pub enum Strategy {
  /// if the difference between two iterations is minimal it stops
  Threshold(f32),

  /// Runs the algorithm a constant number of times
  IterCount(u32),
  // only stop once threshold is below a certain absolute value
  // dangerous if the function is never below that value.
  AbsoluteThreshold(f32),
}

pub(crate) trait StrategyInstance<A> {
  fn should_stop(&self, image: &A) -> bool;
}

impl Strategy {
  pub(crate) fn instance<A>(&self) -> Arc<StrategyInstance<A>> {
    match self {
      _ => unimplemented!(),
    }
  }
}
