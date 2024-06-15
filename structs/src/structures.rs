pub struct Point {
  pub x: i64,
  pub y: i64,
}

impl Point {
  pub fn new(name: &str) -> Point {
    let mut str_x: String = String::new();
    let mut str_y: String = String::new();
    println!("Enter x for {}: ", name);
    std::io::stdin()
      .read_line(&mut str_x)
      .expect("Error reading user input");
    println!("Enter y for {}: ", name);
    std::io::stdin()
      .read_line(&mut str_y)
      .expect("Error reading user input");
    let point: Point = Point {
      x: str_x.as_str().trim().parse().expect("Error parsing to i64"),
      y: str_y.as_str().trim().parse().expect("Error parsing to i64"),
    };
    point
  }
}
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}
pub struct Rectangle {
  pub point1: Point,
  pub point2: Point,
  pub point3: Point,
  pub point4: Point,
}

impl Rectangle {
  pub fn new(point1: Point, point2: Point, point3: Point, point4: Point) -> Rectangle {
    let new_rectangle = Rectangle {
      point1: point1,
      point2: point2,
      point3: point3,
      point4: point4,
    };
    new_rectangle
  }
  pub fn distance(point1: &Point, point2: &Point) -> f64 {
    let x_distance = point1.x - point2.x;
    let y_distance = point1.y - point2.y;
    let total_distance_squared = x_distance * x_distance + y_distance * y_distance;
    let total_distance: f64 = (total_distance_squared as f64).sqrt();
    total_distance
  }
  pub fn area(&self) -> f64 {
    let side_length1 = Rectangle::distance(&self.point1, &self.point2);
    let side_length2 = Rectangle::distance(&self.point2, &self.point3);
    side_length1 * side_length2
  }
  pub fn square(point1: Point, width: i64) -> Rectangle {
    let point1 = Point {
      x: point1.x,
      y: point1.y,
    };
    let point2 = Point {
      x: point1.x,
      y: point1.y - width,
    };
    let point3: Point = Point {
      x: point1.x - width,
      y: point1.y - width,
    };
    let point4: Point = Point {
      x: point1.x - width,
      y: point1.y,
    };
    let rectangle = Rectangle::new(point1, point2, point3, point4);
    rectangle
  }
}

impl std::fmt::Display for Rectangle {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "point1: {}, point2: {}, point3: {}, point4: {}",
      self.point1, self.point2, self.point3, self.point4
    )
  }
}
