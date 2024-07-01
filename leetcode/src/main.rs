mod valid_parentheses;
fn main() {
  println!("{}", valid_parentheses::is_valid(String::from("()[]{}")));
}
