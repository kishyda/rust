pub enum List {
  Cons(u32, Box<List>),
  Nil,
}

impl List {
  pub fn new() -> List {
    List::Nil
  }
  pub fn add(self, value: u32) -> List {
    List::Cons(value, Box::new(self))
  }
  pub fn len(&self) -> u32 {
    match self {
      List::Cons(_, tail) => 1 + tail.len(),
      List::Nil => 0,
    }
  }
  pub fn stringify(&self) -> String {
    match self {
      List::Cons(value, tail) => format!("{}, {}", value, tail.stringify()),
      List::Nil => String::from("nil"),
    }
  }
  pub fn insert(self, index: u32, element: u32) -> List {
    match (self, index) {
      (List::Cons(head, tail), 0) => List::Cons(element, Box::new(List::Cons(head, tail))),
      (List::Cons(head, tail), i) => List::Cons(head, Box::new(tail.insert(i - 1, element))),
      (List::Nil, _) => List::Cons(element, Box::new(List::new())),
    }
  }
  pub fn delete(self, index: u32) -> List {
    match (self, index) {
      (List::Cons(_, tail), 0) => *tail,
      (List::Cons(head, tail), i) => List::Cons(head, Box::new(tail.delete(i - 1))),
      (List::Nil, _) => List::Nil,
    }
  }
}
