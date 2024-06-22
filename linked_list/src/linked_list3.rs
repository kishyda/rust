pub enum List {
  Constructor(i64, Box<List>),
  Nil,
}

impl List {
  pub fn new() -> List {
    Self::Nil
  }
  pub fn add_value(self, element: i64) -> List {
    Self::Constructor(element, Box::new(self))
  }
  pub fn len(&self) -> i64 {
    match self {
      List::Constructor(_, ref tail) => 1 + tail.len(),
      List::Nil => 0,
    }
  }
  pub fn stringify(&self) -> String {
    match self {
      List::Constructor(value, ref tail) => format!("{} {}", value, tail.stringify()),
      List::Nil => format!("nil"),
    }
  }
}
