pub fn is_palindrome(x: i32) -> bool {
  let str: String = x.to_string();
  let mut str2: String = String::new();
  for char in str.chars().rev() {
    str2.push(char);
  }
  if str == str2 {
    true
  } else {
    false
  }
}

pub fn is_palindrome2(x: i32) -> bool {
  let mut x = x;
  let copy = x.clone();
  let mut y: i32 = 0;
  let mut count: i32 = 1;
  loop {
    let placeholder = x % 10;
    y += count * placeholder;
    x = x / 10;
    count *= 10;
    if x == 0 {
      break;
    }
  }
  println!("{} {}", copy, y);
  if copy == y {
    true
  } else {
    false
  }
}
