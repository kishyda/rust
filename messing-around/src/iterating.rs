pub trait Iterator {
  type Item;
  fn any<F>(&mut self, f: F) -> bool
  where
    F: FnMut(Self::Item) -> bool;
}

pub fn function() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];
  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
  println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
}
