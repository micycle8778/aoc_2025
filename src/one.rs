use std::{fs::File, io::Read};

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    dbg!(part_two);
    let mut ret = 0;
    let mut pos = 50;

    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let mut motion = 0;
        if let Some(n) = line.strip_prefix('R').and_then(|s| s.parse::<i32>().ok()) {
            motion = n;
        } else if let Some(n) = line.strip_prefix('L').and_then(|s| s.parse::<i32>().ok()) {
            motion = -n;
        }

        if part_two {
            let loops = (motion / 100).abs();
            ret += loops;

            motion %= 100;

            if motion != 0 {
                let old_pos = pos;
                pos += motion;

                if !(1..100).contains(&pos) && old_pos != 0 {
                    ret += 1;
                }
                pos = pos.rem_euclid(100);
            }
        } else {
            pos += motion;
            pos = pos.rem_euclid(100);

            if pos == 0 {
                ret += 1;
            }
        }
    }

    println!("{ret}");
}
