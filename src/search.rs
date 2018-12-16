extern crate ndarray;

use std::ops::{Mul, Add, Sub, Div};
use std::sync::Arc;
use crate::function::Function;
use crate::grad::grad;
use crate::line_search::{golden_section_search};

use self::ndarray::{Ix1, Ix2};

/// The set of possible strategies to pick from
pub enum Strategy {
  BFGS,
}

struct BFGS<A, S>
  where A: Clone + Add<A, Output=A> + Mul<A, Output=A> + Sub<A, Output=A>,
  S: ndarray::Data<Elem=A> {
  // maintains an internal reference to the hessian approximation
  // and is initialized to the identity matrix TODO
  hessian_approx: ndarray::Array<S, Ix2>,
}

impl <A, S> StrategyInstance<A, S> for BFGS<A, S>
  where
  A: Clone + Mul<A, Output=A> + Add<A, Output=A> + Sub<A, Output=A> + Div<A, Output=A>,
  S: ndarray::Data<Elem = A> {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S,Ix1>,
    f: &Function<A,S>,
    derivative: &Function<A,S>
  ) -> ndarray::ArrayBase<ndarray::OwnedRepr<A>, Ix1> {
    let hess_approx = self.hessian_approx;
    // B_k p_k = - grad f(x);
    // I would imagine that in most cases converting this into an LU system and solving would be
    // good, but wikipedia lists an efficient way to perform the inverse, which should be
    // exploited. TODO
    let curr_grad = grad(f, &curr);
    let direction = unimplemented!();
    let step_size = golden_section_search(f, &curr, &direction);
    let step = direction * step_size;
    let next = curr + &step;
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
pub(crate) trait StrategyInstance <A, S> where S: ndarray::Data<Elem=A> {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S, Ix1>,
    f: &Function<A,S>,
    derivative: &Function<A,S>
  ) -> ndarray::ArrayBase<ndarray::OwnedRepr<A>, Ix1>
  where S: ndarray::Data<Elem=A>;
}

impl Strategy {
   /// returns a specific instance of a strategy so that it can be called in different methods
   /// concurrently
   pub(crate) fn instance<A, S>(&self) -> Arc<StrategyInstance<A, S>>
   where
   A: Clone,
   S: ndarray::Data<Elem=A> {
    unimplemented!();
  }
}
