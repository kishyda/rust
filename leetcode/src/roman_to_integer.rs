enum vals {
  thing(char, i32),
}
pub fn roman_to_int(s: String) -> i32 {
  let mut s = s;
  let mut sum = 0;
  let ss = s.clone();
  for index in 1..ss.len() {
    loop {
      let current_char = s.chars().nth(index);
      let previous_char = s.chars().nth(index - 1);
      match (previous_char, current_char) {
        (Some('I'), Some('V')) => {
          sum += 4;
          s.remove(index);
          s.remove(index - 1);
          continue;
        }
        (Some('I'), Some('X')) => {
          sum += 9;
          s.remove(index);
          s.remove(index - 1);
          continue;
        }
        (Some('X'), Some('L')) => {
          sum += 40;
          s.remove(index);
          s.remove(index - 1);
          continue;
        }
        (Some('X'), Some('C')) => {
          sum += 90;
          s.remove(index);
          s.remove(index - 1);
          continue;
        }
        (Some('C'), Some('D')) => {
          sum += 400;
          s.remove(index);
          s.remove(index - 1);
          continue;
        }
        (Some('C'), Some('M')) => {
          sum += 900;
          s.remove(index);
          s.remove(index - 1);
          continue;
        }
        (_, _) => break,
      }
    }
  }
  for char in s.chars() {
    match char {
      'I' => sum += 1,
      'V' => sum += 5,
      'X' => sum += 10,
      'L' => sum += 50,
      'C' => sum += 100,
      'D' => sum += 500,
      'M' => sum += 1000,
      _ => panic!("something went wrong"),
    }
  }
  sum
}
