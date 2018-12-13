pub enum Strategy {
  Threshold(f32),
  IterCount(u32),
}

impl Strategy {
   pub fn instance(&self) -> StopStrategyInstance {
    unimplemented!();
  }
}
