pub enum Operation {
  Addition { number1: i64, number2: i64 },
  Subtraction(i64, i64),
  Multiplication(i64, i64),
  Division(i64, i64),
}
pub struct Equation {
  pub number1: i64,
  pub number2: i64,
  pub operator: String,
}

pub struct Equation2 {
  number1: i64,
  number2: i64,
}

impl std::fmt::Display for Equation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "(number1): {} (operator): {} (number2): {}",
      self.number1, self.operator, self.number2
    )
  }
}

pub fn parse_argument(string: &mut String) -> Equation {
  let string: &str = string.as_str().trim();
  let mut s_number1: String = String::new();
  let mut s_number2: String = String::new();
  let mut operator: String = String::from('n');
  for char in string.chars() {
    if char == '+' {
      operator.clear();
      operator.push(char);
    } else if char == '-' {
      operator.clear();
      operator.push(char);
    } else if char == '*' {
      operator.clear();
      operator.push(char);
    } else if char == '/' {
      operator.clear();
      operator.push(char);
    } else if operator == String::from('n') {
      s_number1.push(char);
    } else {
      s_number2.push(char);
    }
  }
  let number1: i64 = s_number1.parse().expect("Error parsing string into int");
  let number2: i64 = s_number2.parse().expect("Error parsing string into int");
  Equation {
    number1: number1,
    number2: number2,
    operator: operator,
  }
}

pub fn operate(operation: Operation) -> i64 {
  let result: i64;
  result = match operation {
    Operation::Addition { number1, number2 } => number1 + number2,
    Operation::Subtraction(number1, number2) => number1 - number2,
    Operation::Multiplication(number1, number2) => number1 * number2,
    Operation::Division(number1, number2) => (number1 / number2),
  };
  result
}
