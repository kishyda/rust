pub fn function() {
  let square = |val: i32| -> i32 { val * val };
  let remainder = |val: i32, divisor: i32| -> i32 { val % divisor };
  let square2 = |val: i32| val * val;
  println!("{}", square(5));
  println!("{}", remainder(13, 2));
}