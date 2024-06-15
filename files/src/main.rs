mod create;
mod open;
mod read_lines;

fn main() {
  open::open();
  create::create();
  read_lines::read_lines();
}
