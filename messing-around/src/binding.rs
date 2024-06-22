fn age() -> u32 {
  15
}

fn function() {
  println!("Tell me what type of person you are");
  match age() {
    0 => println!("I haven't celebrated my first birthday yet"),
  }
}
