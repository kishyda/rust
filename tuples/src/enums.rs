enum Color {
  Red,
  Blue,
  Green,
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32),
}
pub fn function() {
  let color = Color::RGB(122, 17, 40);
  println!("What color is it?");
  match color {
    Color::Red => println!("The color is red"),
    Color::Blue => println!("The color is blue"),
    Color::Green => println!("The color is green"),
    Color::RGB(red, blue, green) => println!("red: {} blue {} green {}", red, blue, green),
    Color::HSV(hue, saturation, value) => {
      println!("hue: {}, saturation: {}, value: {}", hue, saturation, value)
    }
    Color::CMY(cyan, magenta, yellow) => {
      println!("cyan: {}, magenta: {}, yellow: {}", cyan, magenta, yellow)
    }
    Color::CMYK(cyan, magenta, yellow, black) => println!(
      "cyan: {}, magenta: {}, yellow: {}, black: {}",
      cyan, magenta, yellow, black
    ),
    _ => println!("Don't ask me..."),
  }
}
