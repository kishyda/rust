use std::io::{Read, Write};

pub fn create() {
  static TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
  let path = std::path::Path::new("Lorem-epsum.txt");
  let display = path.display();

  let mut file = match std::fs::File::create(&path) {
    Ok(value) => value,
    Err(reason) => panic!("Error creating file: {} because {}", display, reason),
  };

  match file.write_all(TEXT.as_bytes()) {
    Ok(_) => println!("Success!"),
    Err(reason) => panic!("error writing to file: {} because {}", display, reason),
  }
  let mut file = match std::fs::File::open(path) {
    Ok(value) => value,
    Err(reason) => panic!("failed to oepn file: {} because {}", display, reason),
  };
  println!("Going to print the file contents...");
  let mut s: String = String::new();
  match file.read_to_string(&mut s) {
    Ok(value) => value,
    Err(reason) => panic!(
      "Failed to write the file: {} to string because {}",
      display, reason
    ),
  };
  println!("{}", s);
}
