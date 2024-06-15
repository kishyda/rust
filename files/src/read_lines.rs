pub fn read_lines() {
  let mut vector: Vec<String> = Vec::new();
  let path = std::path::Path::new("Lorem_epsum.txt");
  let display = path.display();
  let file = match std::fs::File::open(path) {
    Ok(value) => value,
    Err(reason) => panic!("Failed to open file: {} because {}", display, reason),
  };
  for line in std::io::read_to_string(file).unwrap().lines() {
    vector.push(line.to_string());
  }
}
