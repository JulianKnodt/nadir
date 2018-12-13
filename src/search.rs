pub enum Strategy {
  NewtonIteration,
}

impl Strategy {
   pub fn instance(&self) -> SearchStrategyInstance {
    unimplemented!();
  }
}
