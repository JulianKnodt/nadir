extern crate ndarray;

use std::sync::Arc;
use crate::function::Function;
use crate::grad::grad;

/// The set of possible strategies to pick from
pub enum Strategy {
  BFGS,
}

// minimizes f as a function of t in
// f(start + t*direction)
// used in BFGS
// reference: https://en.wikipedia.org/wiki/Golden-section_search
fn golden_section_search<S, D>(
  f: &Function<S, D>,
  start: &ndarray::ArrayBase<S, D>,
  direction: &ndarray::ArrayBase<S,D>) -> S
  where S: ndarray::Data {
  // TODO do something.
  unimplemented!();
}

struct BFGS<S, D> {
  // maintains an internal reference to the hessian approximation
  // and is initialized to the identity matrix TODO
  hessian_approx: ndarray::Array<S, D>,
}

impl <S, D> StrategyInstance<S, D> for BFGS<S, D> where S: ndarray::Data {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S, D>,
    curr_deriv: &ndarray::ArrayBase<S, D>,
    f: &Function<S, D>,
    derivative: &Function<S, D>
  ) -> ndarray::ArrayBase<S, D> {
    // need to find where to get the initial hessian approximation
    let hess_approx = self.hessian_approx;
    // need to solve B_k p_k = - grad f(x);
    // I would imagine that in most cases converting this into an LU system and solving would be
    // good, but wikipedia lists an efficient way to perform the inverse, which should be
    // exploited. TODO
    let direction = unimplemented!();
    let step_size = golden_section_search(f, curr, direction);
    let step = step_size * direction;
    let next = curr + step;
    let update = next - curr;

    let next_hessian =
      hess_approx
      + (update * update.t()/(update.t() * step))
      - (hess_approx * step * step.t() * hess_approx)/(step.t() * hess_approx * step);
    self.hessian_approx = next_hessian;

    next
  }
}

/// The generic trait for all instances of strategies, for predicting the next value of
/// iteration
pub(crate) trait StrategyInstance <S, D> where S: ndarray::Data {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S, D>,
    curr_deriv: &ndarray::ArrayBase<S, D>,
    f: &Function<S, D>,
    derivative: &Function<S, D>
  ) -> ndarray::ArrayBase<S, D>;
}

impl Strategy {
   /// returns a specific instance of a strategy so that it can be called in different methods
   /// concurrently
   pub(crate) fn instance<S, D>(&self) -> Arc<StrategyInstance<S, D>>
   where S: ndarray::Data {
    unimplemented!();
  }
}
