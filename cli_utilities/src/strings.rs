pub fn string(string: &mut String) -> &mut String {
  let mut string2: &mut String = string;
  println!("Enter string");
  std::io::stdin()
    .read_line(&mut string2)
    .expect("Error reading string");
  string2
}
