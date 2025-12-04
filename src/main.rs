#![forbid(clippy::pedantic)]
use std::env;
use std::fs::File;


fn main() {
    let day = env::args().nth(1);
    let filename = env::args().nth(2);

    if let [Some(day), Some(filename)] = [day, filename] {
        let Ok(mut file) = File::open(filename) else {
            eprintln!("error opening file. does it exist?");
            return;
        };

        match day.as_str() {
            "one" => aoc_2025::one::solution(&mut file),
            _ => eprintln!("i dont have a solution for that day")
        }
    } else {
        eprintln!("USAGE: cargo run <day> <filename>");
    }
}
