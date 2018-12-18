extern crate ndarray;

use std::ops::{Mul, Add, Sub, Div};
use std::sync::Arc;
use crate::function::{Function, FunctionGradient};
use crate::line_search::{golden_section_search};

use self::ndarray::{Ix, Ix1, Ix2};

/// The set of possible strategies to pick from
pub enum Strategy {
  BFGS,
}

struct BFGS<A>
  where A: Clone + Add<A, Output=A> + Mul<A, Output=A> + Sub<A, Output=A> {
  hessian_approx: ndarray::Array<A, Ix2>,
  inverse_approx: ndarray::Array<A, Ix2>,
}

impl <A, S> StrategyInstance<A, S> for BFGS<A>
  where
  A: Clone + Mul<A, Output=A> + Add<A, Output=A> + Sub<A, Output=A> + Div<A, Output=A> +
  Div<f64, Output=A> + PartialOrd + ndarray::ScalarOperand,
  S: ndarray::Data<Elem = A> {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S,Ix1>,
    f: &Function<A,S>,
    grad: &FunctionGradient<A,S>
  ) -> ndarray::Array<A,Ix1> {
    // B_k p_k = - grad f(x);
    // I would imagine that in most cases converting this into an LU system and solving would be
    // good, but wikipedia lists an efficient way to perform the inverse, which should be
    // exploited.
    let curr_grad = grad(curr);
    let direction:ndarray::Array<A,Ix1> = curr_grad * self.inverse_approx;
    let step_size = golden_section_search(f, curr, &direction);
    let step: ndarray::Array<A,Ix1> = direction * step_size;
    let next = curr + &step;
    let update = &next - curr;
    let update_t = update.t().to_owned();
    let step_t = step.t().to_owned();
    let test = ndarray::ArrayBase::eye(update.dim()) * update;

    let hess_approx = &self.hessian_approx;
    let next_hessian = hess_approx + (update * &update_t)/(update_t * &step)
      - (hess_approx * &step * &step_t * hess_approx)/(&step_t * hess_approx * &step);
    self.hessian_approx = next_hessian;

    let inv_approx = &self.inverse_approx;
    let next_inv = unimplemented!();
    self.inverse_approx = next_inv;

    return next;
  }
}

/// The generic trait for all instances of strategies, for predicting the next value of
/// iteration
pub(crate) trait StrategyInstance <A, S> where S: ndarray::Data<Elem=A> {
  fn predict(&mut self,
    curr: &ndarray::ArrayBase<S, Ix1>,
    f: &Function<A,S>,
    grad: &FunctionGradient<A,S>
  ) -> ndarray::Array<A, Ix1>
  where S: ndarray::Data<Elem=A>;
}

extern crate num_traits;
use self::num_traits::identities::{Zero, One};
impl Strategy {
  /// returns a specific instance of a strategy so that it can be called in different methods
  /// concurrently
  pub(crate) fn instance<A, S>(&self, dim: Ix) -> Arc<StrategyInstance<A, S>>
  where
  A: Clone + Mul<A, Output=A> + Add<A, Output=A> + Sub<A, Output=A> + Div<A, Output=A> +
  One + Zero + Div<f64, Output=A> + PartialOrd + ndarray::ScalarOperand,
  S: ndarray::Data<Elem=A> {
    match self {
      Strategy::BFGS => Arc::new(BFGS{
        hessian_approx: ndarray::ArrayBase::eye(dim),
        inverse_approx: ndarray::ArrayBase::eye(dim),
      }),
    }
  }
}
