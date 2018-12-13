pub enum Strategy {
  Threshold(f32),
  IterCount(u32),
}

trait StrategyInstance {

}

impl Strategy {
   pub fn instance(&self) -> StrategyInstance {
    unimplemented!();
  }
}
