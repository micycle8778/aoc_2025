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

        let part_two = day.ends_with('2');

        match day.strip_suffix('2').unwrap_or(day.as_str()) {
            "one" => aoc_2025::one::solution(&mut file, part_two),
            "two" => aoc_2025::two::solution(&mut file, part_two),
            "five" => aoc_2025::five::solution(&mut file, part_two),
            "six" => aoc_2025::six::solution(&mut file, part_two),
            _ => eprintln!("i dont have a solution for that day")
        }
    } else {
        eprintln!("USAGE: cargo run <day> <filename>");
    }
}
