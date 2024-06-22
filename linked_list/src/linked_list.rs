pub enum List {
  Cons(u32, Box<List>),
  Nil,
}

impl List {
  pub fn new() -> List {
    List::Nil
  }
  pub fn add(self, element: u32) -> List {
    List::Cons(element, Box::new(self))
  }
  pub fn len(&self) -> u32 {
    match *self {
      List::Cons(_, ref tail) => 1 + tail.len(),
      List::Nil => 0,
    }
  }
  pub fn stringify(&self) -> String {
    match *self {
      List::Cons(head, ref tail) => {
        format!("{}, {}", head, tail.stringify())
      }
      List::Nil => {
        format!("Nil")
      }
    }
  }
  pub fn insert(self, index: u32, element: u32) -> List {
    match (self, index) {
      (List::Nil, _) => List::Cons(element, Box::new(List::Nil)),
      (List::Cons(value, tail), 0) => List::Cons(element, Box::new(List::Cons(value, tail))),
      (List::Cons(value, tail), i) => List::Cons(value, Box::new((tail.insert(i - 1, element)))),
    }
  }
  pub fn delete(self, index: u32) -> List {
    match (self, index) {
      (List::Nil, _) => List::Nil,
      (List::Cons(_, tail), 0) => *tail,
      (List::Cons(value, tail), i) => List::Cons(value, Box::new(tail.delete(i - 1))),
    }
  }
}
