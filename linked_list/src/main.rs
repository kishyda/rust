mod linked_list;
mod linked_list2;
mod linked_list3;
mod self_implemented_linked_list;
mod string_list;

fn function1() {
  let mut list = linked_list::List::new();
  list = list.add(1);
  list = list.add(2);
  list = list.add(3);
  println!("linked list has length: {}", list.len());
  println!("{}", list.stringify());
  list = list.insert(2, 9);
  println!("{}", list.stringify());
  list = list.delete(3);
  println!("{}", list.stringify());
}

fn function2() {
  let mut list = linked_list2::List::new();
  list = list.add(1);
  list = list.add(2);
  list = list.add(3);
  println!("linked list has length: {}", list.len());
  println!("{}", list.stringify());
}
fn function3() {
  let mut list = linked_list3::List::new();
  list = list.add_value(1);
  list = list.add_value(2);
  list = list.add_value(3);
  println!("linked list has length: {}", list.len());
  println!("{}", list.stringify());
}
fn string_function() {
  let mut list = string_list::StringList::new();
  list = list.add(String::from("bro"));
  list = list.add(String::from("bro"));
  list = list.add(String::from("bro"));
  list = list.add(String::from("bro"));
  println!("{}", list.stringify());
}

fn my_implementation() {
  let mut list = self_implemented_linked_list::List::new();
  list = list.add(1);
  list = list.add(1);
  list = list.add(1);
  list = list.add(1);
  list = list.add(1);
  list = list.add(1);
  list = list.add(1);
  list = list.add(1);
  list = list.insert(3, 2);
  list = list.delete(3);
  list = list.delete(3);
  list = list.delete(3);
  list = list.delete(3);
  list = list.delete(3);
  list = list.insert(3, 2);
  list = list.insert(3, 2);
  list = list.insert(3, 2);
  list = list.insert(3, 2);
  println!("{}, {}", list.stringify(), list.len());
}
fn main() {
  my_implementation();
}
