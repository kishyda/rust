pub fn function() {
  let array = [1, -2, 6];

  match array {
    // [1, _, third] => println!(
    //   "first value: {}, second value don't matter, third value: {}",
    //   1, third
    // ),
    [1, ..] => println!("first value is {}, the rest don't matter", 1),
    _ => println!("don't ask me"),
  }
}
