use std::sync::Arc;

pub enum Strategy {
  Threshold(f32),
  IterCount(u32),
}

pub(crate) trait StrategyInstance<S> where S: ndarray::Data {
  fn should_stop(&self, image: &S) -> bool;
}

impl Strategy {
   pub(crate) fn instance<S: ndarray::Data>(&self) -> Arc<StrategyInstance<S>> {
    unimplemented!();
  }
}
