pub struct vector(pub Vec<i32>);

impl std::fmt::Display for vector {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let vec = &self.0;
    write!(f, "vector is: [")?;
    for (count, v) in vec.iter().enumerate() {
      if count != 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}", v)?;
    }
    write!(f, "]")
  }
}
