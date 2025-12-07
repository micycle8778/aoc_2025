use std::{fs::File, io::Read};

struct StateMachine {
    state: Box<[u8]>,
    splits: u32
}

impl StateMachine {
    fn new(initial: &[u8]) -> Self {
        Self {
            state: initial.into(),
            splits: 0
        }
    }

    fn advance(&mut self, next: &[u8]) {
        let mut new_state = vec![b'.'; next.len()].into_boxed_slice();

        for idx in 0..new_state.len() {
            if new_state[idx] == b'|' {
                continue;
            }

            let n = next[idx];
            let s = self.state[idx];

            // println!("n = {}; s = {}", n as char, s as char);

            match n {
                b'.' => {
                    if [b'|', b'S'].contains(&s) {
                        new_state[idx] = b'|';
                    } else {
                        new_state[idx] = n;
                    }
                }

                b'^' => {
                    if s == b'|' {
                        new_state[idx - 1] = b'|';
                        new_state[idx] = n;
                        new_state[idx + 1] = b'|';
                        self.splits += 1;
                    }
                }

                _ => unreachable!()
            }
        }

        self.state = new_state;
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let lines: Vec<_> = buf.lines().map(str::as_bytes).collect();
    let mut sm = StateMachine::new(lines[0]);

    for line in &lines[1..] {
        println!("{}", str::from_utf8(&sm.state).unwrap());
        sm.advance(line);
    }
    println!("{}", str::from_utf8(&sm.state).unwrap());
    println!("{}", sm.splits);
}
