fn name() {}

fn parse_argument(args: Vec<String>) {
  for index in 0..args.len() {
    match args[index].as_str() {
      "name" => println!("Error"),
      _ => print!("Error"),
    }
  }
}
