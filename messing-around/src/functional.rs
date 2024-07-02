pub fn functional() {
  let thing: i32 = (0..)
    .map(|x| x * x)
    .take_while(|&x| x < 1000)
    .filter(|&x| x % 2 == 0)
    .sum();
  println!("{}", thing);
}

pub fn functional2() {
  let bruh: Vec<i128> = (0..)
    .map(|x| x * x * x * x * x * x)
    .take_while(|&x| x < 100000000000000000)
    .collect();
  println!("{:?}", bruh);
}
