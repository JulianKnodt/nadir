// TODO built the gradient of a function

// reference: https://www.mathworks.com/help/matlab/ref/gradient.html
extern crate ndarray;

use crate::function::Function;

/// Takes a function and returns its gradient at x, in the form of a column vector.
pub(crate) fn grad<S, D>(f: &Function<S, D>, x: &ndarray::ArrayBase<S, D>)
  -> ndarray::Array<S, D>
  where S: ndarray::Data {
  unimplemented!();
}
