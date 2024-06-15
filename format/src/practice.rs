#[derive(Debug)]
pub struct SomeRandomStructure {
  pub x: i64,
  pub y: i64,
  pub z: i64,
}

impl std::fmt::Display for SomeRandomStructure {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "x = {}, y = {}, z = {}", self.x, self.y, self.z)
  }
}
