use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn open() {
  let path = Path::new("hello.txt");
  let display = path.display();
  let mut file = match File::open(path) {
    Ok(value) => value,
    Err(reason) => panic!("{} was not opened because {}", display, reason),
  };
  let mut s: String = String::new();
  match file.read_to_string(&mut s) {
    Ok(value) => value,
    Err(reason) => panic!(
      "Reading file: {} to string failed because {}",
      display, reason
    ),
  };
  println!("{}", s);
}
