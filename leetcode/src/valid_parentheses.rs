pub fn is_valid(s: String) -> bool {
  let mut str1 = String::new();
  let mut str2 = String::new();
  for char in s.chars() {
    match char {
      '(' => str1.push(char),
      '{' => str1.push(char),
      '[' => str1.push(char),
      _ => continue,
    }
  }
  println!("{}", str1);
  for char in str1.chars().rev() {
    match char {
      '(' => str2.push(')'),
      '{' => str2.push('}'),
      '[' => str2.push(']'),
      _ => continue,
    }
  }
  println!("{}", str2);
  let finalstring = str1 + &str2;
  println!("{}", finalstring);
  if finalstring == s {
    true
  } else {
    false
  }
}
