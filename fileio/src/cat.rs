use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

pub fn openfile(filepath: &str) {
  let path = Path::new(filepath);
  let display = path.display();

  let mut file = match File::open(filepath) {
    Err(reason) => panic!("Error reading {} because {}", display, reason),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(reason) => panic!("Failed to read {} because {}", filepath, reason),
    Ok(_) => println!("Success!"),
  }
  println!("{}", s);
}
