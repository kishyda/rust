mod practice;
mod practice2;
mod practice3;
mod practice4;
#[derive(Debug)]
struct Point {
  x: i64,
  y: i64,
}
struct MyInt(i32);
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}, {}", self.x, self.y)
  }
}

impl std::fmt::Display for MyInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "bro... the answer is {}", self.0)
  }
}

fn main() {
  let point = Point { x: 3, y: 5 };
  let strct = practice::SomeRandomStructure { x: 3, y: 4, z: 6 };
  let hello = practice2::HelloThere {
    name: String::from("Yusuf"),
    age: 19,
  };
  println!("{}", strct);
  println!("{:?}", strct);
  println!("{}", hello);
  let intvalue = MyInt(3);
  println!("{}", intvalue);
  let stupidValues = practice3::StupidStruct {
    a: 1,
    b: 2,
    c: 3,
    d: 4,
    e: 5,
  };
  println!("{}", stupidValues);
  println!("{:?}", stupidValues);
  let vector = practice4::vector(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  println!("{}", vector);
}
