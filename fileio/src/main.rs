mod echo;
mod cat;
mod ls;
mod find;
mod grep;
use clap::Parser;
use std::env;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(long, default_value_t = String::from(""))]
  echo: String,
  #[arg(long, default_value_t = String::from(""))]
  cat: String,
  #[arg(long, default_value_t = String::from(""))]
  ls: String,
  #[arg(long, default_value_t = String::from(""))]
  find: String,
  #[arg(long, default_value_t = String::from(""))]
  grep: String,
}
fn main() {
  let args = Args::parse();
  match args.echo.as_str() {
    "" => println!("Doing nothing"),
    _ => {
      let args2: Vec<String> = env::args().collect();
      println!("--ECHO--");
      for i in 0..args2.len() {
        print!("{} ", args2[i]);
      }
    }
  }
  println!("--CAT--");
  match args.cat.as_str() {
    "" => println!("Doing nothing"),
    _ => cat::openfile(args.cat.as_str()),
  }
  match args.ls.as_str() {
    "" => println!("Doing nothing"),
    _ => 
  }
}
