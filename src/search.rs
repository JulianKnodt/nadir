pub enum Strategy {
  NewtonIteration,
}

trait StrategyInstance {
}

impl Strategy {
   pub fn instance(&self) -> StrategyInstance {
    unimplemented!();
  }
}
