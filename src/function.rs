extern crate ndarray;

type Function<S, D> = fn(x: ndarray::ArrayBase<S, D>) -> S;

pub fn find_min<S , D>(f: &Function<S, D>, start: ndarray::ArrayBase<S, D>)
  -> ndarray::ArrayBase<S, D> where S: ndarray::Data {
  unimplemented!();
}
