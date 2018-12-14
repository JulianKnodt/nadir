use std::sync::Arc;

pub enum Strategy {
  Threshold(f32),
  IterCount(u32),
  // only stop once threshold is below a certain absolute value
  // dangerous if the function is never below that value.
  AbsoluteThreshold(f32),
}

pub(crate) trait StrategyInstance<S> where S: ndarray::Data {
  fn should_stop(&self, image: &S) -> bool;
}

struct AbsoluteThreshold(f32);

impl StrategyInstance<S> for AbsoluteThreshold where S: ndarray::Data {
  fn should_stop(&self, image: &S) -> bool {
    unimplemented!();
    // upon inpection it appears that ndarray::Data does not have a norm property
  }
}

impl Strategy {
  pub(crate) fn instance<S: ndarray::Data>(&self) -> Arc<StrategyInstance<S>> {
    match self {
      Strategy::AbsoluteThreshold(f32) => AbsoluteThreshold(f32),
      _ => unimplemented!(),
    }
  }
}
