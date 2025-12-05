use std::{fs::File, io::Read};
use std::ops::{Range, RangeInclusive};

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut ret = 0;
    let mut ranges = Vec::<RangeInclusive<u64>>::new();

    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    
    let mut first_pass = true;
    for line in buf.lines() {
        if line.is_empty() { 
            first_pass = false;
            continue;
        }

        if first_pass {
            let mut it = line.split('-').map(|s| s.parse::<u64>().unwrap());
            let left = it.next().unwrap();
            let right = it.next().unwrap();

            println!("{left}-{right}");

            ranges.push(left..=right);
        } else {
            let n = line.parse::<u64>().unwrap();
            for r in &ranges {
                if r.contains(&n) {
                    ret += 1;
                    break;
                }
            }
        }
    }
    
    println!("{ret}");
}

