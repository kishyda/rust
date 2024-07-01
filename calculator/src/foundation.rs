pub fn take_string_input() -> String {
  let mut temp: String = String::new();
    .read_line(&mut temp)
    .expect("error reading file");
  temp
}

pub fn parse_to_int(string: String) -> i64 {
  let str: &str = string.as_str();
  let num: i64 = str
    .trim()
    .parse()
    .expect("Error parsing from string to int");
  num
}

pub fn take_equation() -> (i64, char, i64, String) {
  let num1: i64;
  let num2: i64;
  let mut string1: String = String::new();
  let mut string2: String = String::new();
  let mut operator: char = '\0';
  println!("Enter equation");
  let mut stringinput: String = String::from(take_string_input().trim());
  let mut flag = 0;
  let mut loopcount = 0;
  for char in stringinput.chars().into_iter() {
    match char {
      '0'..='9' if flag == 0 => {
        string1.push(char);
        // println!("first num: {}, {}", char, flag);
        loopcount += 1;
      }
      oper if oper == '+' || oper == '-' || oper == '*' || oper == '/' || flag == 0 => {
        // println!("operator: {}, {}", char, flag);
        operator = oper;
        flag += 1;
        loopcount += 1;
      }
      '0'..='9' if flag == 1 => {
        string2.push(char);
        // println!("second num: {}, {}", char, flag);
        loopcount += 1;
      }
      _ => break,
    }
  }
  num1 = string1.parse().expect("error parsing string to int");
  num2 = string2.parse().expect("error parsing string to int");
  for index in 0..loopcount - 1 {
    stringinput.remove(0);
  }
  (num1, operator, num2, stringinput)
}

pub fn solve(num1: i64, operator: char, num2: i64) -> i64 {
  match operator {
    '+' => num1 + num2,
    '-' => num1 - num2,
    '*' => num1 * num2,
    '/' => num1 / num2,
    _ => panic!("something went wrong while solving"),
  }
}

pub fn take_equation2(num1: i64, rest_of_string: String) -> (i64, String) {
  let mut rest_of_string = String::from(rest_of_string.trim());
  let operator: char = rest_of_string.chars().nth(0).expect("");
  let mut num2string = String::new();
  let num2: i64;
  let mut count = 1;
  for char in rest_of_string.chars().skip(0) {
    if char == '+' || char == '-' || char == '/' || char == '*' {
      break;
    } else {
      num2string.push(char);
      count += 1;
    }
  }
  for index in 0..count {
    rest_of_string.remove(0);
  }
  num2 = num2string.parse().expect("");
  let result = solve(num1, operator, num2);
  (result, rest_of_string)
}

pub fn solve_equation() -> i64 {
  let (num1, operator, num2, rest_of_string) = take_equation();
  let rest_of_string = rest_of_string;
  let answer: i64;
  let x = solve(num1, operator, num2);
  loop {
    let (x, rest_of_string) = take_equation2(x, rest_of_string.to_owned());
    if rest_of_string == String::from("") {
      answer = x;
      break;
    }
  }
  answer
}
