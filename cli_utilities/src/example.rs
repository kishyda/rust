use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[arg(short, long)]
    age: u8,
}
fn main() {
    let args = Args::parse();
    for index in 0..args.count {
        println!("Name is {}, age is {}", args.name, args.age);
    }
}
