use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file = &args[1];

    let contents = fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        println!("{}", line);
    }
}
