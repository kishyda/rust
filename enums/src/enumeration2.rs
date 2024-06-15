#[derive(Debug)]
pub enum WebEvent2 {
  Click,
  Open,
  Bruh(i64),
  Yo(String, i64),
  Cool(String, i64, f64),
}

pub fn tryingthingsout() {
  let click = WebEvent2::Click;
  let open = WebEvent2::Open;
  let bruh = WebEvent2::Bruh(15);
  let yo = WebEvent2::Yo(String::from("Hello there"), 39);
  let cool = WebEvent2::Cool(String::from("cool"), 12, 213.232);
  println!("{:?}", click);
  println!("{:?}", open);
  println!("{:?}", bruh);
  println!("{:?}", yo);
  println!("{:?}", cool);
}
