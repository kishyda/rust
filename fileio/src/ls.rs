use std::fs;
use std::io;
use std::path::Path;

fn list_directory(path: &str) -> io::Result<()> {
  let path = Path::new(path);
  if path.is_dir() {
    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        println!("{}/", path.display());
      } else {
        println!("{}", path.display());
      }
    }
  } else {
    println!("{} is not a directory.", path.display());
  }
  Ok(())
}