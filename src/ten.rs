use std::io::Read;
use std::fs::File;

use itertools::Itertools;
use regex::Regex;

struct Machine {
    goal: Vec<bool>,
    ops: Vec<Vec<usize>>,
    joltage: Vec<usize>
}

impl Machine {
    fn solve(&self) -> u32 {
        // BFS of our state space
        let mut states = vec![(vec![false; self.goal.len()], 0)];

        loop {
            let mut new_states = Vec::with_capacity(states.len() * self.ops.len());

            for s in states {
                for op in &self.ops {
                    let mut new_s = (s.0.clone(), s.1 + 1);
                    for &idx in op {
                        new_s.0[idx] = !new_s.0[idx];
                    }

                    if new_s.0 == self.goal {
                        dbg!(op);
                        // hoorah!
                        return new_s.1;
                    }
                    else {
                        new_states.push(new_s);
                    }
                }
            }
            
            states = new_states;
        }
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut ret = 0;

    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let goal_re = Regex::new(r"\[(.*)\]").unwrap();
    let ops_re = Regex::new(r"\((.*?)\)").unwrap();
    let joltage_re = Regex::new(r"\{(.*?)\}").unwrap();

    for line in buf.lines() {
        let goal_str = &goal_re.captures(line).unwrap()[1];
        let goal = goal_str.bytes().map(|b| 
            if b == b'.' { false }
            else if b == b'#' { true }
            else { unreachable!() }
        ).collect_vec();

        let mut ops = Vec::new();
        for op in ops_re.captures_iter(line) {
            ops.push(op[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect_vec());
        }

        let joltage_str = &joltage_re.captures(line).unwrap()[1];
        let joltage = joltage_str.split(',').map(|s| s.parse::<usize>().unwrap()).collect_vec();

        ret += dbg!(Machine { goal, ops, joltage }.solve());
    }

    dbg!(ret);
}
