extern crate ndarray;

use crate::{search, stop};

use self::ndarray::Ix1;

/// Type alias of functions that we are expecting.
pub type Function<A, S> where S: ndarray::Data<Elem=A> =
  fn(x: &ndarray::ArrayBase<S, Ix1>) -> A;

/// Finds the global minimum of some function f and it's derivative, given starting position
/// "start", and using search and stop strategies as provided.
pub fn find_min<A, S>(
  f: &Function<A,S>,
  derivative: &Function<A,S>,
  start: ndarray::ArrayBase<S, Ix1>,
  search_strat: search::Strategy,
  stop_strat: stop::Strategy
  ) -> (ndarray::ArrayBase<S, Ix1>, A)
  where
  A: Clone,
  S: ndarray::Data<Elem=A> + Clone + ndarray::DataClone {
  let stop_inst = stop_strat.instance();
  let search_inst = search_strat.instance();
  let mut curr = start.clone();
  let mut curr_image: A = f(&curr);
  while !stop_inst.should_stop(&curr_image) {
    curr = search_inst.predict(&curr, f, derivative);
    curr_image = f(&curr);
  }
  (curr, curr_image)
}
