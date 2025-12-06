#![allow(clippy::cast_possible_wrap)]
use std::{fs::File, io::Read};

fn solution_rec(nums: &[u64], digits_left: u32, acc: u64) -> u64 {
    if digits_left == 0 {
        acc
    } else if digits_left as usize == nums.len() {
        solution_rec(&nums[1..], digits_left - 1, acc + nums[0] * 10u64.pow(digits_left - 1))
    } else {
        let mut pairs = nums[0..=nums.len() - digits_left as usize].iter()
            .enumerate()
            .collect::<Vec<_>>();
        pairs.sort_by_key(|&(_, &n)| -(n as i64));

        let biggest_n = *pairs[0].1;
        pairs.into_iter()
            .filter(|(_, n)| **n == biggest_n)
            .map(|(idx, &n)| solution_rec(&nums[idx + 1..], digits_left - 1, acc + n * 10u64.pow(digits_left - 1)))
            .max().unwrap()
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    println!("{}",
        buf.lines()
            .map(|line| line.chars().map(|c| c.to_string().parse::<u64>().unwrap()).collect::<Box<[_]>>())
            .map(|nums| solution_rec(&nums, if part_two { 12 } else { 2 }, 0))
            .sum::<u64>()
    );
}
