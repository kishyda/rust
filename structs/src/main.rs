mod structures;

fn main() {
  let point1 = structures::Point::new("point1");
  let point2 = structures::Point::new("point2");
  let point3 = structures::Point::new("point3");
  let point4 = structures::Point::new("point4");
  let rectangle = structures::Rectangle::new(point1, point2, point3, point4);
  println!("{}", rectangle);
  println!("area is {}", rectangle.area());
  let point = structures::Point::new("Upper left corner point");
  let square = structures::Rectangle::square(point, 5);
  println!("{}", square);
}
