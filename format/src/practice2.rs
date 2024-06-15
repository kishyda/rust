use std::default;

#[derive(Debug, Default)]
pub struct HelloThere {
  pub name: String,
  pub age: u8,
}
impl HelloThere {
  fn HelloThere(&mut self) {
    self.name = "".to_string();
    self.age = 0;
  }
}

impl std::fmt::Display for HelloThere {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Bro's name is {}, bro's age is {}", self.name, self.age)
  }
}
