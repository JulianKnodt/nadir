extern crate ndarray;

use crate::{search, stop};

type Function<S, D> = fn(x: ndarray::ArrayBase<S, D>) -> S;

pub enum SearchStrategy {
  NewtonIteration,
}

pub fn find_min<S , D>(f: &Function<S, D>, derivative: &Function<S, D>,
  start: ndarray::ArrayBase<S, D>, search_strat: search::Strategy, stop_strat: stop::Strategy)
  -> (ndarray::ArrayBase<S, D>, S) where S: ndarray::Data {
  assert!(is_column_vector(start));
  unimplemented!();
  let stop_inst = stop_strat.instance();
  let search_inst = search_strat.instance();
  let mut curr = start;
  let mut curr_image = f(curr);
  while !stop_inst.should_stop(curr_image) {
    curr  = search_inst.evaluate(curr, curr_image, f, derivative);
    curr_image = f(curr);
  }
  (curr, curr_image)
}
