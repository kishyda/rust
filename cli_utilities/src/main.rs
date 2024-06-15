use clap::Parser;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value_t = String::from(""))]
  cat: String,
  #[arg(long, default_value_t = String::from(""))]
  echo: String,
  #[arg(long, default_value_t = String::from(""))]
  ls: String,
}

fn concatenation() {
  let mut usrinput: String = String::new();
  println!("Enter file path");
  std::io::stdin()
    .read_line(&mut usrinput)
    .expect("Error reading string");
  let path = Path::new(usrinput.trim());
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("Couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why),
    Ok(_) => print!("{} contains:\n{}", display, s),
  }
  println!("{}", s);
}

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
fn main() {
  let args = Args::parse();
  if args.cat != "" {
    concatenation();
  }
  if args.echo == "true" {
    let args2: Vec<String> = std::env::args().collect();
    println!("{:?}", args2);
  }
  if args.ls != "" {
    let copy: &str = args.ls.as_str();
    list_directory(copy).expect("Not working");
  }
}
