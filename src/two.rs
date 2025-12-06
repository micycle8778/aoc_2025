#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

use std::{cell::Cell, collections::HashSet, fs::File, io::Read, ops::RangeFrom};

type R = std::ops::RangeInclusive<u64>;

#[derive(Clone)]
struct FillIterator {
    internal: RangeFrom<u64>,
    digits: u32,
}

impl FillIterator {
    fn new(digits: u32) -> Self {
        Self {
            internal: (1..),
            digits
        }
    }
}

impl Iterator for FillIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let src = self.internal.next().unwrap();
            let src_digits = src.ilog10() + 1;

            if src_digits > self.digits / 2 {
                return None;
            }

            if self.digits % src_digits != 0 {
                continue;
            }

            let n = self.digits as usize / src_digits as usize;
            let ret = src.to_string().repeat(n).parse::<u64>().unwrap();
            // println!("{src} {} {ret}", self.digits);
            return Some(ret);
        }
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut ranges = Vec::<R>::new();
    let mut ret = 0;

    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    // generate ranges
    for s in buf.split(',') {
        let mut it = s.split('-').map(|s| s.trim().parse::<u64>().unwrap());
        let left = it.next().unwrap();
        let right = it.next().unwrap();

        ranges.push(left..=right);
    }

    if part_two {
        for r in ranges {
            let mut seen = HashSet::<u64>::new();

            let digits_start = r.start().ilog10() + 1;
            let digits_end = r.end().ilog10() + 1;

            let it = FillIterator::new(digits_start).chain(FillIterator::new(digits_end));
            
            for n in it.clone() {
                if r.contains(&n) && !seen.contains(&n) {
                    // println!("{n}");
                    seen.insert(n);
                    ret += n;
                }
            }

            // println!("{} {digits_start} {} {digits_end} {:?}", r.start(), r.end(), it.last());
        }
    } else {
        for r in ranges {
            let str_rep = r.start().to_string();
            let n = (str_rep.len() as f32 / 2.).ceil();

            let doubled = Cell::new((*r.start() / 10u64.pow(n as u32)).max(10u64.pow(n as u32 - 1)));
            let do_double = || doubled.get() + doubled.get() * 10u64.pow(doubled.get().ilog10() + 1);

            // println!("{str_rep} {n} {} {}", doubled.get(), do_double());

            loop {
                if r.contains(&do_double()) {
                    // println!("{}", do_double());
                    ret += do_double();
                } else if do_double() > *r.start() {
                    break;
                }
                doubled.set(doubled.get() + 1);
            }
        }
    }

    println!("{ret}");
}
