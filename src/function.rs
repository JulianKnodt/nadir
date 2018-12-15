extern crate ndarray;

use crate::{search, stop};

/// Type alias of functions that we are expecting.
pub type Function<S, D> = fn(x: &ndarray::ArrayBase<S, D>) -> S;

// tests whether a function is a column vector
// just to ensure that we get what we expect.
fn is_column_vector<S:ndarray::Data, D: ndarray::Dimension>(s: &ndarray::ArrayBase<S, D>)
  -> bool {
  s.ndim() == 1
}

/// Finds the global minimum of some function f and it's derivative, given starting position
/// "start", and using search and stop strategies as provided.
pub fn find_min<A, S, D>(f: &Function<S, D>, derivative: &Function<S, D>,
  start: ndarray::ArrayBase<S, D>, search_strat: search::Strategy, stop_strat: stop::Strategy)
  -> (ndarray::ArrayBase<S, D>, S)
  where S: ndarray::Data + Clone + Copy + ndarray::DataClone,
  D: ndarray::Dimension + Copy + Clone {
  assert!(is_column_vector(&start));
  let stop_inst = stop_strat.instance();
  let search_inst = search_strat.instance();
  let mut curr = start.clone();
  let mut curr_image = f(&curr);
  while !stop_inst.should_stop(&curr_image) {
    curr  = search_inst.predict(&curr, f, derivative);
    curr_image = f(&curr);
  }
  (curr, curr_image)
}
