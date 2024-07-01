pub struct Equation {
  pub num1: i64,
  pub num2: i64,
  pub operator: char,
  pub rest_of_string: String,
}
pub fn take_input() -> String {
  let mut string: String = String::new();
  std::io::stdin()
    .read_line(&mut string)
    .expect("error reading string");
  string
}

pub fn string_to_equation(string: String) -> Equation {
  let mut string = string;
  let mut num1 = String::new();
  let mut num2 = String::new();
  let mut operator: char = 'n';
  let mut flag = 0;
  let mut loopcount = 0;
  for char in string.chars() {
    match char {
      '0'..='9' if flag == 0 => {
        num1.push(char);
        loopcount += 1;
      }
      n if (char == '*' || char == '/' || char == '+' || char == '-') && flag == 0 => {
        operator = n;
        loopcount += 1;
        flag += 1;
      }
      '0'..='9' if flag == 1 => {
        num2.push(char);
        loopcount += 1;
      }
      _ => break,
    }
  }
  let num1: i64 = num1.parse().expect("");
  let num2: i64 = num2.parse().expect("");
  for index in 0..loopcount {
    string.remove(0);
  }
  Equation {
    num1: num1,
    num2: num2,
    operator: operator,
    rest_of_string: string,
  }
}
pub fn solve_first_equation(equation: Equation) -> (i64, String) {
  match equation.operator {
    '+' => (equation.num1 + equation.num2, equation.rest_of_string),
    '-' => (equation.num1 - equation.num2, equation.rest_of_string),
    '*' => (equation.num1 * equation.num2, equation.rest_of_string),
    '/' => (equation.num1 / equation.num2, equation.rest_of_string),
    _ => panic!("something went wrong"),
  }
}

pub fn solve(num1: i64, operator: char, num2: i64) -> i64 {
  match operator {
    '+' => num1 + num2,
    '-' => num1 - num2,
    '*' => num1 * num2,
    '/' => num1 / num2,
    _ => panic!("error at the solve function"),
  }
}
pub fn read_operator(string: String) -> char {
  let operator = string.chars().nth(0).expect("Error parsing string to char");
  operator
}

pub fn solve_equation(num1: i64, string: String) -> (i64, String) {
  let mut string = string;
  let mut num2: String = String::new();
  let operator = string.chars().nth(0).expect("error reading operator");
  let mut loopcount = 1;
  for char in string.chars().skip(1) {
    match char {
      '0'..='9' => {
        num2.push(char);
        loopcount += 1;
      }
      n if (n == '+' || n == '*' || n == '/' || n == '-') => break,
      _ => (),
    }
  }
  for index in 0..loopcount {
    string.remove(0);
  }
  let result = solve(
    num1,
    operator,
    num2.parse().expect("Error parsing from string to int"),
  );
  (result, string)
}

pub fn loop_solve_equation(num: i64, string: String) -> i64 {
  let mut num: i64 = num;
  let mut string: String = string;
  loop {
    if string == "" || string == "\n" || string == "\t" {
      break;
    } else {
      (num, string) = solve_equation(num, string)
    }
  }
  num
}

pub fn wrapper_function() -> i64 {
  let string = take_input();
  let equation = string_to_equation(string);
  let (num1, string) = solve_first_equation(equation);
  loop_solve_equation(num1, string)
}
