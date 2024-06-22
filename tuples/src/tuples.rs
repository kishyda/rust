pub fn function() {
  let triple = (0, -2, 3);
  match triple {
    // (0, y, z) => println!("first is '0', 'y', is {:?}, and 'z' is {:?}", y, z),
    (0, -2, _) => println!("Working "),
    _ => println!("don't ask me"),
  }
}
