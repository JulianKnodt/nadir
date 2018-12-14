extern crate ndarray;

use std::sync::Arc;
use crate::function::Function;

pub enum Strategy {
  NewtonIteration,
}

pub(crate) trait StrategyInstance <S, D> where S: ndarray::Data {
  fn predict(&self, curr: &ndarray::ArrayBase<S, D> , f: &Function<S, D>,
    derivative: &Function<S, D>) -> ndarray::ArrayBase<S, D>;
}

impl Strategy {
   pub(crate) fn instance<S, D>(&self) -> Arc<StrategyInstance<S, D>>
   where S: ndarray::Data {
    unimplemented!();
  }
}
