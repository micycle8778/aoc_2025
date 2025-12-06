use std::{fs::File, io::Read};
use std::ops::RangeInclusive;

type R = RangeInclusive<u64>;
trait RangeExt {
    fn contains_range(&self, other: &Self) -> bool;
}

impl RangeExt for R {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

/// # Panics 
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut ret = 0;
    let mut ranges = Vec::<R>::new();

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

            ranges.push(left..=right);
        } else if part_two {
            let mut better_ranges = Vec::<R>::new();

            ranges.sort_by_key(|r| *r.start());
            for r in ranges {
                'label: {
                    // see if we can modify a range before pushing it to `better_ranges`
                    for br in &mut better_ranges {
                        if br.contains_range(&r) {
                            // r is within br :(
                            break 'label;
                        } else if br.contains(r.start()) && !br.contains(r.end()) { // r can extend br to the right
                            *br = *br.start()..=*r.end();
                            break 'label;
                        } else if br.contains(r.end()) && !br.contains(r.start()) { // r can extend br to left
                            *br = *r.start()..=*br.end();
                            break 'label;
                        } else if r.contains_range(br) { // r contains br!
                            *br = r;
                            break 'label;
                        }
                    }

                    better_ranges.push(r);
                }
            }

            better_ranges.sort_by_key(|r| *r.start());
            for br in better_ranges {
                println!("{}-{}", br.start(), br.end());
                ret += br.end() - br.start() + 1;
            }

            break;
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

