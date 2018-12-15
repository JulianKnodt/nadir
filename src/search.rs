extern crate ndarray;

use std::ops::{Mul, Add, Sub, Div};
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
fn golden_section_search<A, S, D>(
  f: &Function<A,S,D>,
  start: &ndarray::ArrayBase<S, D>,
  direction: &ndarray::ArrayBase<S,D>) -> A
  where A: Clone,
  S: ndarray::Data<Elem = A> {
  // TODO
  unimplemented!();
}

struct BFGS<A, S, D>
  where A: Clone + Add<A, Output=A> + Mul<A, Output=A> + Sub<A, Output=A>,
  S: ndarray::Data<Elem=A> {
  // maintains an internal reference to the hessian approximation
  // and is initialized to the identity matrix TODO
  hessian_approx: ndarray::Array<S, D>,
}

impl <A, S, D> StrategyInstance<A, S, D> for BFGS<A, S, D>
  where
  A: Clone + Mul<A, Output=A> + Add<A, Output=A> + Sub<A, Output=A> + Div<A, Output=A>,
  S: ndarray::Data<Elem = A>,
  D: ndarray::Dimension {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S,D>,
    f: &Function<A,S,D>,
    derivative: &Function<A,S,D>
  ) -> ndarray::ArrayBase<S, D> {
    // need to find where to get the initial hessian approximation
    let hess_approx = self.hessian_approx;
    // need to solve B_k p_k = - grad f(x);
    // I would imagine that in most cases converting this into an LU system and solving would be
    // good, but wikipedia lists an efficient way to perform the inverse, which should be
    // exploited. TODO
    let curr_grad = grad(f, &curr);
    let direction = unimplemented!();
    let step_size = golden_section_search(f, &curr, direction);
    let step = direction * step_size;
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
pub(crate) trait StrategyInstance <A, S, D> where S: ndarray::Data<Elem=A> {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S, D>,
    f: &Function<A,S,D>,
    derivative: &Function<A,S,D>
  ) -> ndarray::ArrayBase<S, D> where S: ndarray::Data<Elem=A>;
}

impl Strategy {
   /// returns a specific instance of a strategy so that it can be called in different methods
   /// concurrently
   pub(crate) fn instance<A, S, D>(&self) -> Arc<StrategyInstance<A, S, D>>
   where
   A: Clone,
   S: ndarray::Data<Elem=A> {
    unimplemented!();
  }
}
