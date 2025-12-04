use std::{fs::File, io::Read};

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File) {
    let mut ret = 0;
    let mut pos = 50;

    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        if let Some(n) = line.strip_prefix('R').and_then(|s| s.parse::<i32>().ok()) {
            pos += n;
        } else if let Some(n) = line.strip_prefix('L').and_then(|s| s.parse::<i32>().ok()) {
            pos -= n;
        }

        pos = pos.rem_euclid(100);
        if pos == 0 {
            ret += 1;
        }
    }

    println!("{ret}");
}
