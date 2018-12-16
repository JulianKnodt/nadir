// Line search is the general term for searching in one dimension

extern crate ndarray;

use std::ops::{Div, Sub};
use crate::function::Function;
use self::ndarray::Ix1;

const phi : f64 = (5.0f64.sqrt() + 1.0)/2.0;

/// minimizes f as a function of t in
/// f(start + t*direction)
/// reference: https://en.wikipedia.org/wiki/Golden-section_search
pub fn golden_section_search<A, S>(
  f: &Function<A,S>,
  start: &ndarray::ArrayBase<S,Ix1>,
  direction: &ndarray::ArrayBase<S,Ix1>) -> A
  where
  A: Clone + PartialOrd + Div<f64, Output = A> + Sub<A, Output=A>,
  S: ndarray::Data<Elem = A> {

  // Realized this was WRONG but its close to being right, TODO
  unimplemented!();

  // need some way to find the left and right bounds
  // let (mut left, mut right) : (A, A) = unimplemented!();

  // while (left-right).abs() > tolerance {
  //   let a = right - (right -left)/phi;
  //   let b = left + (right - left)/phi;
  //   // TODO reuse function calls
  //   if f(a) < f(b) { right = b } else { left = a };
  // }
  // left
}
