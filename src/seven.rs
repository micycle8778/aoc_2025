use std::{fs::File, io::Read};

mod part_one {
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

    pub fn solution(lines: &[&[u8]]) {
        let mut sm = StateMachine::new(lines[0]);

        for line in &lines[1..] {
            println!("{}", str::from_utf8(&sm.state).unwrap());
            sm.advance(line);
        }
        println!("{}", str::from_utf8(&sm.state).unwrap());
        println!("{}", sm.splits);
    }
}

mod part_two {
    use std::ops::AddAssign;
    use std::ops::Add;

    #[derive(Clone, Copy)]
    enum State {
        Splitter,
        Count(u64)
    }

    impl State {
        fn count(&self) -> u64 {
            match *self {
                Self::Splitter => 0,
                Self::Count(n) => n
            }
        }
    }

    impl Add for State {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            if let Self::Splitter = self  { panic!(); }
            if let Self::Splitter = rhs   { panic!(); }
            
            Self::Count(self.count() + rhs.count())
        }
    }

    impl AddAssign for State {
        fn add_assign(&mut self, rhs: Self) {
            if let Self::Splitter = self  { panic!(); }
            if let Self::Splitter = rhs   { panic!(); }
            
            *self = *self + rhs;
        }
    }

    impl From<u8> for State {
        fn from(value: u8) -> Self {
            match value {
                b'^' => Self::Splitter,
                b'S' => Self::Count(1),
                _ => Self::Count(0),
            }
        }
    }

    trait SliceExt {
        fn chars(&self) -> impl Iterator<Item = char>;
    }

    impl SliceExt for [State] {
        fn chars(&self) -> impl Iterator<Item = char> {
            self.iter().map(|s| match *s {
                State::Splitter => '^',
                State::Count(n) => {
                    if n >= 16 {
                        'F'
                    } else {
                        format!("{:x}", n).chars().next().unwrap()
                    }
                }
            })
        }
    }

    struct StateMachine {
        state: Box<[State]>,
    }

    impl StateMachine {
        fn new(initial: &[u8]) -> Self {
            let state = initial.iter().copied().map(State::from).collect();
            Self {
                state,
            }
        }

        fn advance(&mut self, next: &[u8]) {
            let mut new_state = vec![State::Count(0); next.len()].into_boxed_slice();

            for idx in 0..new_state.len() {
                // if new_state[idx] == b'|' {
                //     continue;
                // }

                let n = State::from(next[idx]);
                let s = self.state[idx];

                match n {
                    State::Count(_) => new_state[idx] = State::Count(s.count() + new_state[idx].count()),
                    State::Splitter => {
                        new_state[idx - 1] += State::Count(s.count());
                        new_state[idx] = State::Splitter;
                        new_state[idx + 1] += State::Count(s.count());
                    }
                }
            }

            self.state = new_state;
        }
    }

    pub fn solution(lines: &[&[u8]]) {
        let mut sm = StateMachine::new(lines[0]);

        for line in &lines[1..] {
            println!("{}", sm.state.chars().collect::<String>());
            sm.advance(line);
        }
        println!("{}", sm.state.chars().collect::<String>());
        println!("{}", sm.state.iter().map(|s| s.count()).sum::<u64>());
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let lines: Vec<_> = buf.lines().map(str::as_bytes).collect();

    if part_two {
        part_two::solution(&lines);
    } else {
        part_one::solution(&lines);
    }
}
