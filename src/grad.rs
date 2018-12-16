// TODO built the gradient of a function

// reference: https://www.mathworks.com/help/matlab/ref/gradient.html
extern crate ndarray;

use std::ops::{Add, Mul, Div, Sub};

use crate::function::Function;
use self::ndarray::{Ix1};

/// Takes a function and returns its gradient at x, in the form of a column vector.
/// h is the parameter for calculating the secant around x.
/// implemented using symmetric difference quotient, which is numerically better than
/// (f(x+h) - f(x))/h
pub(crate) fn grad<A,S>(f: &Function<A,S>, x: &ndarray::ArrayBase<S,Ix1>)
  -> ndarray::Array<ndarray::OwnedRepr<A>, Ix1>
  where
  A: Clone + Add<A, Output = A> + ndarray::ScalarOperand + Add<f64, Output=A> +
  Mul<f64, Output=A> + Div<A, Output=A> + Sub<A, Output=A>,
  S: ndarray::Data<Elem=A> {
  let h = 0.000000001;
  ndarray::Array::from_vec(x.iter().enumerate().map(|(idx, el)| {
    // (f(x+h)-f(x-h))/2h
    (f(&(x+h)) - f(&(x-h)))/(h * 2.0)
  }).collect())
}
