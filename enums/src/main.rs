mod calculator;
mod enumeration2;
mod enumeration3;
mod enumerations;
mod trolling;
fn main() {
  let pressed = enumerations::WebEvent::KeyPress('x');
  enumerations::inspect(pressed);
  let mut input: String = String::new();
  std::io::stdin()
    .read_line(&mut input)
    .expect("error reading line");
  let equation: calculator::Equation = calculator::parse_argument(&mut input);
  let x = match equation.operator.as_str() {
    "+" => calculator::Operation::Addition {
      number1: equation.number1,
      number2: equation.number2,
    },
    "-" => calculator::Operation::Subtraction(equation.number1, equation.number2),
    "*" => calculator::Operation::Multiplication(equation.number1, equation.number2),
    "/" => calculator::Operation::Division(equation.number1, equation.number2),
    _ => panic!("Error parsing equation into operator"),
  };
  let result = calculator::operate(x);
  println!("{}", result);
  enumeration2::tryingthingsout();
  enumeration3::run();
}
