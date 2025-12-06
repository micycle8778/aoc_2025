use std::{fs::File, io::Read};

struct StateMachine {
    string_buf: String,
    number_buf: Vec<u64>,
    ret: u64
}

impl StateMachine {
    fn new() -> Self {
        Self {
            string_buf: String::new(),
            number_buf: Vec::new(),
            ret: 0
        }
    }

    fn advance(&mut self, c: char) {
        if c.is_numeric() {
            self.string_buf.push(c);
        } else {
            if !self.string_buf.is_empty() {
                self.number_buf.push(self.string_buf.parse::<u64>().unwrap());
                // self.number_buf.push(self.string_buf.chars().rev().collect::<String>().parse().unwrap());
                self.string_buf.clear();
            }

            match c {
                '*' => {
                    self.ret += self.number_buf.iter().product::<u64>();
                    self.number_buf.clear();
                }
                '+' => {
                    self.ret += self.number_buf.iter().sum::<u64>();
                    self.number_buf.clear();
                }
                _ => {}
            }
        }
    }
}

fn solution2(buf: &str) {
    let mut sm = StateMachine::new();
    let n_lines = buf.lines().count();
    let n_cols = buf.lines().next().unwrap().len();

    let buf2 = buf.chars().filter(|x| *x != '\n').collect::<String>();

    for col in (0..n_cols).rev() {
        for line in 0..n_lines {
            let idx = (line * n_cols) + col;
            let c = buf2.chars().nth(idx).unwrap_or_else(|| panic!("character should exist at index {idx}"));
            sm.advance(c);
        }
    }

    println!("{}", sm.ret);
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut ret = 0u64;


    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    if part_two {
        solution2(&buf);
        return;
    }

    let mut nses: Vec<Vec<u64>> = Vec::new();
    let lines: Vec<&str> = buf.lines().collect();

    for line in &lines[..lines.len() - 1] {
        let flag = nses.is_empty();
        for (idx, n) in line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).enumerate() {
            if flag {
                nses.push(vec![n]);
            } else {
                nses[idx].push(n);
            }
        }
    }

    for (operation, ns) in lines[lines.len() - 1].split_whitespace().zip(nses) {
        match operation {
            "*" => ret += ns.iter().product::<u64>(),
            "+" => ret += ns.iter().sum::<u64>(),
            _ => panic!("caught unexpected operation {operation}")
        }
    }

    println!("{ret}");
}
