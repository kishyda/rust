pub enum StringList {
  Constructor(String, Box<StringList>),
  Nil,
}

impl StringList {
  pub fn new() -> StringList {
    StringList::Nil
  }
  pub fn add(self, element: String) -> StringList {
    StringList::Constructor(element, Box::new(self))
  }
  pub fn stringify(&self) -> String {
    match self {
      Self::Constructor(value, ref tail) => format!("{}, {}", value, tail.stringify()),
      Self::Nil => format!("nil"),
    }
  }
}
